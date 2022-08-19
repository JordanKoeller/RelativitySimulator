use std::io::Error;
use std::result::Result;
use std::sync::Arc;
use std::thread;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tokio::runtime;
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

use super::{
    ActorMessage, Connection, ConnectionId, ConnectionManager, ConnectionParameters, ConnectionStatus, SocketMessage,
    SocketMessageEnum, AcceptedConnection
};
use crate::events::ReceiverID;

pub struct Actor {
    thread_handle: Option<thread::JoinHandle<()>>,
    inbox: mpsc::UnboundedSender<ActorMessage<SocketMessage>>,
    outbox: mpsc::UnboundedReceiver<SocketMessage>,
    connection_manager: Arc<ConnectionManager>,
}

impl Actor {
    pub fn new() -> Self {
        let runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Could not construct tokio runtime");

        let (actor_send, mut actor_recv) = mpsc::unbounded_channel::<ActorMessage<SocketMessage>>();
        let (back_send, back_recv) = mpsc::unbounded_channel::<SocketMessage>();

        let manager = Arc::from(ConnectionManager::new());

        let async_manager = Arc::clone(&manager);

        let handle = thread::spawn(move || {
            runtime.block_on(async move {
                let mut senders: Vec<mpsc::UnboundedSender<SocketMessage>> = Vec::new();
                while let Some(task) = actor_recv.recv().await {
                    match task {
                        ActorMessage::Shutdown => {
                            Self::shutdown(&senders);
                            break;
                        }
                        ActorMessage::SendMessage(msg, id) => {
                            tokio::spawn(Self::send_message(msg, senders[id.channel()].clone()));
                        }
                        // TODO: Properly handle TCPClient, TCPServer, and UDP
                        ActorMessage::ConnectTo(cx) => {
                            let (send, recv) = mpsc::unbounded_channel::<SocketMessage>();
                            senders.push(send);
                            tokio::spawn(Self::connect_to(
                                cx,
                                recv,
                                back_send.clone(),
                                Arc::clone(&async_manager),
                            ));
                        }
                        ActorMessage::DropConnection(cx) => {
                            async_manager.drop_channel(cx.channel());
                        }
                    }
                }
                println!("Networking actor finished");
            });
        });
        Self {
            thread_handle: Some(handle),
            inbox: actor_send,
            connection_manager: manager,
            outbox: back_recv,
        }
    }

    pub fn connect(
        &self,
        connection_parameters: ConnectionParameters,
        receiver_id: ReceiverID,
    ) -> Option<ConnectionId> {
        let channel_id = self.connection_manager.get_channel_id();
        let connection_id = ConnectionId::new(receiver_id, channel_id);
        let connection = connection_parameters.to_connection(connection_id);
        self.inbox
            .send(ActorMessage::ConnectTo(connection))
            .ok()
            .map(|_| connection_id)
    }

    pub fn send(&mut self, message: SocketMessage, id: ConnectionId) {
        let _result = self.inbox.send(ActorMessage::SendMessage(message, id));
    }

    pub fn recv(&mut self) -> Option<SocketMessage> {
        match self.outbox.try_recv() {
            Ok(msg) => Some(msg),
            _ => None,
        }
    }
}

// Helper methods that run on the consumer thread
impl Actor {
    async fn send_message(msg: SocketMessage, channel: mpsc::UnboundedSender<SocketMessage>) {
        channel.send(msg).ok();
    }

    async fn connect_to(
        connection_parameters: Connection,
        mut inbound_channel: mpsc::UnboundedReceiver<SocketMessage>,
        outbound_channel: mpsc::UnboundedSender<SocketMessage>,
        connection_manager: Arc<ConnectionManager>,
    ) -> std::result::Result<(), std::io::Error> {
        let connection_id = connection_parameters.id();
        connection_manager.set_channel_status(connection_id.channel(), ConnectionStatus::Pending);
        let socket = TcpStream::connect(connection_parameters.connection_string()).await?;
        let (mut reader, mut writer) = tokio::io::split(socket);

        connection_manager.set_channel_status(connection_id.channel(), ConnectionStatus::Stable);

        tokio::spawn(async move {
            while let Some(msg) = inbound_channel.recv().await {
                writer.write_all(msg.data()).await?;
            }
            Ok::<_, std::io::Error>(())
        });

        // let mut buf = vec![0; 128];
        let byte_size = 128usize;
        loop {
            let mut msg = SocketMessage::with_capacity(connection_parameters.id(), byte_size);
            let _read_bytes_count = reader.read(msg.data_mut()).await?;
            outbound_channel.send(msg).ok();
        }
        Ok(())
    }

    async fn tcp_server(
        connection_parameters: Connection,
        connection_manager: Arc<ConnectionManager>,
        outbound_channel: mpsc::UnboundedReceiver<AcceptedConnection>
    ) -> Result<(), Error> {
        let server_connection_id = connection_parameters.id();
        connection_manager.set_channel_status(server_connection_id.channel(), ConnectionStatus::Pending);

        let server = TcpListener::bind(connection_parameters.connection_string()).await?;

        while let Some((socket, client_info)) = server.accept().await.ok() {
            
        }

        Ok(())

    }

    fn shutdown(connections: &Vec<mpsc::UnboundedSender<SocketMessage>>) {
        connections.iter().for_each(|connection| {
            connection.send(SocketMessage::shutdown_message()).ok();
        });
    }
}

impl Drop for Actor {
    fn drop(&mut self) {
        self.inbox.send(ActorMessage::Shutdown).ok();
        self.thread_handle.take().map(|handle| handle.join());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_make_actor() {
        let _actor = Actor::new();
    }
}
