use std::collections::HashMap;
use std::io::{Error, IoSlice};
use std::result::Result;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use serde::{Deserialize, Serialize};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{
    tcp::{OwnedReadHalf, OwnedWriteHalf},
    TcpListener, TcpStream,
};
use tokio::runtime;
use tokio::spawn as dispatch;
use tokio::sync::mpsc::{
    channel as async_channel, unbounded_channel, Receiver as AsyncReceiver, Sender as AsyncSender, UnboundedReceiver,
    UnboundedSender,
};
use tokio::sync::watch::{channel as watch_channel, Receiver as WatchReceiver, Sender as WatchSender};

use super::{
    ActorMessage, Connection, ConnectionId, ConnectionManager, ConnectionParameters, ConnectionStatus, EgressMessage,
    Envelope, NewConnectionMessage, SocketType,
};
use crate::events::ReceiverID;

const HEADER_SIZE: u32 = 4;

pub struct NetActor {
    ingress_receiver: UnboundedReceiver<ActorMessage>,
    ingress_sender: UnboundedSender<ActorMessage>,
    egress_sender: Sender<Envelope>,
    egress_send_connection: Sender<NewConnectionMessage>,
    connection_manager: Arc<ConnectionManager>,
    connection_senders: Vec<Option<AsyncSender<Envelope>>>,
}

impl NetActor {
    pub fn create() -> (Self, NetActorHandle) {
        let (ingress_sender, ingress_receiver) = unbounded_channel::<ActorMessage>();
        let (egress_sender, egress_receiver) = channel();
        let (egress_send_connection, egress_recv_connection) = channel();
        let connection_manager = Arc::from(ConnectionManager::new());
        let handle = NetActorHandle::new(
            Arc::clone(&connection_manager),
            ingress_sender.clone(),
            egress_receiver,
            egress_recv_connection,
        );
        let actor = Self {
            ingress_receiver,
            ingress_sender,
            egress_sender,
            egress_send_connection,
            connection_manager,
            connection_senders: Vec::default(),
        };
        (actor, handle)
    }

    // Entrypoint for starting up the NetActor in a separate thread
    pub fn execute(mut self) {
        let runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Could not construct tokio runtime");

        std::thread::spawn(move || runtime.block_on(self.handle_events()));
    }

    // Main event-handling method
    async fn handle_events(&mut self) -> Result<(), Error> {
        while let Some(message) = self.ingress_receiver.recv().await {
            match message {
                ActorMessage::AcceptConnection {
                    stream,
                    host,
                    new_client,
                } => {
                    let new_connection_id =
                        ConnectionId::new(host.receiver(), self.connection_manager.get_channel_id());
                    let new_connection = Connection::new(new_client, new_connection_id);
                    self.setup_duplex(stream, new_connection_id);
                    self.egress_send_connection
                        .send(NewConnectionMessage::new_subconnection(new_connection, host));
                }
                ActorMessage::FinalizeConnection { stream, connection } => {
                    self.setup_duplex(stream, connection.id());
                    self.egress_send_connection.send(NewConnectionMessage::new(connection));
                }
                ActorMessage::HostConnection(connection) => {
                    self.connection_manager
                        .set_channel_status(connection.id().channel(), ConnectionStatus::Stable);
                    tokio::spawn(Self::establish_connection_host(connection, self.ingress_sender.clone()));
                }
                ActorMessage::ConnectTo(connection) => {
                    tokio::spawn(Self::connect_to_host(connection, self.ingress_sender.clone()));
                }
                ActorMessage::SendMessage(envelope) => {
                    match self.connection_senders[envelope.connection_id.channel()].as_ref() {
                        Some(sender) => {
                            sender.send(envelope).await;
                        }
                        None => {}
                    }
                }
                // TODO: Proper cleanup
                ActorMessage::DropConnection(connection_id) => {}
                ActorMessage::Shutdown => return Ok(()),
            };
        }
        Ok(())
    }
}

// Synchronous Helper methods
impl NetActor {
    fn setup_connection(&mut self, connection_id: ConnectionId) -> AsyncReceiver<Envelope> {
        while self.connection_senders.len() <= connection_id.channel() {
            self.connection_senders.push(None);
        }
        let (tx, rx) = async_channel(128);
        self.connection_senders[connection_id.channel()] = Some(tx);
        rx
    }

    fn setup_duplex(&mut self, stream: TcpStream, connection_id: ConnectionId) {
        let rx = self.setup_connection(connection_id);
        let (tcp_reader, tcp_writer) = stream.into_split();
        tokio::spawn(Self::establish_duplex_send(tcp_writer, rx));
        tokio::spawn(Self::establish_duplex_recv(
            tcp_reader,
            connection_id,
            self.egress_sender.clone(),
        ));
        self.connection_manager
            .set_channel_status(connection_id.channel(), ConnectionStatus::Stable);
    }
}

// Async Helper methods that should be ran in separately spawned futures
impl NetActor {
    // async fn spawn

    async fn establish_duplex_send(stream: OwnedWriteHalf, mut receiver: AsyncReceiver<Envelope>) -> Result<(), Error> {
        while let Some(envelope) = receiver.recv().await {
            let envelope_size = envelope.data.len() as u32;
            let size_bytes = envelope_size.to_le_bytes();
            let packet = &[IoSlice::new(&size_bytes), IoSlice::new(&envelope.data)];
            stream.try_write_vectored(packet).ok();
        }
        Ok(())
    }

    async fn establish_duplex_recv(
        mut stream: OwnedReadHalf,
        connection_id: ConnectionId,
        back_sender: Sender<Envelope>,
    ) -> Result<(), Error> {
        loop {
            let mut size_bytes: [u8; 4] = [0, 0, 0, 0];
            stream.read_exact(&mut size_bytes).await?;
            let packet_size = u32::from_le_bytes(size_bytes);
            let mut packet_buffer = Vec::with_capacity(packet_size as usize);
            for _ in 0..packet_size {
                packet_buffer.push(0);
            }
            stream.read_exact(&mut packet_buffer).await?;
            let response = Envelope::new(packet_buffer, connection_id);
            back_sender.send(response);
        }
    }

    // TODO: Add a way to shut down the host.
    // This method sets up a TCPListener to act as a host, waiting for clients to connect
    async fn establish_connection_host(
        connection: Connection,
        ingress_sender: UnboundedSender<ActorMessage>,
    ) -> Result<(), Error> {
        let tcp_listener = TcpListener::bind(connection.connection_string()).await?;
        while let Some((tcp_stream, client_info)) = tcp_listener.accept().await.ok() {
            let path = client_info.to_string();
            let port = client_info.port();
            ingress_sender.send(ActorMessage::AcceptConnection {
                stream: tcp_stream,
                new_client: ConnectionParameters::new_tcp_client(&path, port as u32),
                host: connection.id(),
            });
        }
        Ok(())
    }

    // TODO: Properly error handle
    // Acting as the client, request to connect to a remote host and wait for the connection to finish.
    async fn connect_to_host(
        connection: Connection,
        ingress_sender: UnboundedSender<ActorMessage>,
    ) -> Result<(), Error> {
        let tcp_stream = TcpStream::connect(connection.connection_string()).await?;
        ingress_sender.send(ActorMessage::FinalizeConnection {
            stream: tcp_stream,
            connection: connection.clone(),
        });
        Ok(())
    }
}

pub struct NetActorHandle {
    connections: Arc<ConnectionManager>,
    ingress_channel: UnboundedSender<ActorMessage>,
    egress_channel: Receiver<Envelope>,
    egress_recv_connection: Receiver<NewConnectionMessage>,
    pending_new_connections: HashMap<ConnectionId, Vec<Connection>>,
}

impl NetActorHandle {
    fn new(
        connections: Arc<ConnectionManager>,
        ingress_channel: UnboundedSender<ActorMessage>,
        egress_channel: Receiver<Envelope>,
        egress_recv_connection: Receiver<NewConnectionMessage>,
    ) -> Self {
        Self {
            connections,
            ingress_channel,
            egress_channel,
            egress_recv_connection,
            pending_new_connections: HashMap::new(),
        }
    }
    pub fn connection_manager(&self) -> &Arc<ConnectionManager> {
        &self.connections
    }

    pub fn send_message<T: Serialize + Deserialize<'static>>(&self, connection_id: ConnectionId, obj: T) {
        let data = serde_json::to_vec(&obj).unwrap();
        self.send(ActorMessage::SendMessage(Envelope::new(data, connection_id)));
    }

    pub fn read_message_raw(&mut self) -> Option<Envelope> {
        self.egress_channel.try_recv().ok()
    }

    pub fn read_message<T>(&mut self) -> Option<T>
    where
        for<'a> T: Serialize + Deserialize<'a>,
    {
        self.egress_channel
            .try_recv()
            .ok()
            .map(|envelope| serde_json::from_slice(&envelope.data).ok())
            .flatten()
    }

    pub fn get_new_connections(&mut self, host_connection_id: &ConnectionId) -> Option<Vec<Connection>> {
        self.process_new_connections();
        self.pending_new_connections.remove(host_connection_id)
    }

    pub fn shutdown(&self) {
        self.ingress_channel.send(ActorMessage::Shutdown).ok();
    }
    pub fn create_connection(&self, parameters: ConnectionParameters, receiver_id: ReceiverID) -> ConnectionId {
        let connection_id = ConnectionId::new(receiver_id, self.connection_manager().get_channel_id());
        self.connection_manager()
            .set_channel_status(connection_id.channel(), ConnectionStatus::Pending);
        let connection = Connection::new(parameters, connection_id);
        match connection.connection_type() {
            SocketType::TCPClient => self.send(ActorMessage::ConnectTo(connection)),
            SocketType::TCPServer => self.send(ActorMessage::HostConnection(connection)),
            SocketType::UDP => println!("UDP not supported!"),
        };
        connection_id
    }

    pub fn get_connection_status(&self, id: ConnectionId) -> ConnectionStatus {
        self.connections.get_channel_status(id.channel())
    }

    pub fn ready(&self, id: ConnectionId) -> bool {
        self.get_connection_status(id) == ConnectionStatus::Stable
    }

    fn send(&self, message: ActorMessage) {
        self.ingress_channel.send(message).ok();
    }

    fn process_new_connections(&mut self) {
        self.egress_recv_connection.try_iter().for_each(|new_connection| {
            let (parent_id_opt, connection) = new_connection.unpack();
            let (k, v) = if let Some(parent_id) = parent_id_opt {
                (parent_id, connection)
            } else {
                (connection.id(), connection)
            };
            if self.pending_new_connections.contains_key(&k) {
                self.pending_new_connections
                    .get_mut(&k)
                    .map(|connections| connections.push(v));
            } else {
                self.pending_new_connections.insert(k, vec![v]);
            }
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_make_netactor() {
        let (_actor, _handle) = NetActor::create();
    }

    #[test]
    fn can_start_netactor() {
        let (actor, handle) = NetActor::create();

        actor.execute();

        thread::sleep_ms(3000);

        handle.shutdown();
    }

    #[test]
    fn can_start_and_connect_host() {
        let (actor, mut handle) = NetActor::create();

        actor.execute();

        thread::sleep_ms(500);

        let host_id = handle.create_connection(ConnectionParameters::new_tcp_host("localhost", 8080), 0);

        thread::sleep_ms(500);

        let client_id = handle.create_connection(ConnectionParameters::new_tcp_client("localhost", 8080), 0);

        thread::sleep_ms(500);

        let msg = vec![42, 42, 42, 42];

        handle.send_message(client_id, msg);

        thread::sleep_ms(500);

        let response = handle.read_message_raw();

        assert_eq!(response.is_some(), true);

        handle.shutdown();
    }

    #[test]
    fn can_get_connection_satus() {
        let (actor, mut handle) = NetActor::create();

        actor.execute();
        let host_id = handle.create_connection(ConnectionParameters::new_tcp_host("localhost", 8080), 0);
        assert_eq!(handle.get_connection_status(host_id), ConnectionStatus::Pending);
        thread::sleep_ms(500);
        assert_eq!(handle.get_connection_status(host_id), ConnectionStatus::Stable);
        let client_id = handle.create_connection(ConnectionParameters::new_tcp_client("localhost", 8080), 0);
        assert_eq!(handle.get_connection_status(client_id), ConnectionStatus::Pending);
        thread::sleep_ms(500);
        assert_eq!(handle.get_connection_status(client_id), ConnectionStatus::Stable);

        thread::sleep_ms(500);

        handle.shutdown();
    }

    #[test]
    fn can_send_arbitrary_packet_sizes() {
        let (actor, mut handle) = NetActor::create();

        // Start connection
        actor.execute();
        thread::sleep_ms(500);
        let host_id = handle.create_connection(ConnectionParameters::new_tcp_host("localhost", 8080), 0);
        thread::sleep_ms(500);
        let client_id = handle.create_connection(ConnectionParameters::new_tcp_client("localhost", 8080), 0);
        thread::sleep_ms(500);

        // Create a message
        let mut msg: Vec<u8> = Vec::with_capacity(123usize);
        for i in 0..msg.len() {
            msg.push(((i * 2 + 12) % 255) as u8);
        }

        // tx/rx
        handle.send_message(client_id, msg.clone());
        thread::sleep_ms(500);
        let response = handle.read_message_raw();

        // assert
        assert_eq!(response.is_some(), true);
        assert_eq!(response.unwrap().data, msg);

        //cleanup
        handle.shutdown();
    }

    #[test]
    fn can_get_new_connection_notification() {
        let (actor, mut handle) = NetActor::create();

        // Start connection
        actor.execute();
        thread::sleep_ms(500);
        let host_id = handle.create_connection(ConnectionParameters::new_tcp_host("localhost", 8080), 0);
        thread::sleep_ms(500);
        let client_id = handle.create_connection(ConnectionParameters::new_tcp_client("localhost", 8080), 0);
        thread::sleep_ms(500);

        let new_connections = handle.get_new_connections(&host_id);
        assert_eq!(new_connections.is_some(), true);
        let connections = new_connections.unwrap();
        assert_eq!(connections.len(), 1);
    }
}
