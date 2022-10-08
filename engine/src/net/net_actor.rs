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
  Envelope, GenericConnectionId, NetActorHandle, NetEvent, NewConnectionMessage, SocketType,
};
use crate::events::ReceiverId;

const HEADER_SIZE: u32 = 4;

pub struct NetActor {
  ingress_receiver: UnboundedReceiver<ActorMessage>,
  ingress_sender: UnboundedSender<ActorMessage>,
  egress_sender: Sender<NetEvent>,
  connection_manager: Arc<ConnectionManager>,
  connection_senders: Vec<Option<AsyncSender<Envelope>>>,
}

impl NetActor {
  pub fn create() -> (Self, NetActorHandle) {
    let (ingress_sender, ingress_receiver) = unbounded_channel::<ActorMessage>();
    let (egress_sender, egress_receiver) = channel();
    let connection_manager = Arc::from(ConnectionManager::new());
    let handle = NetActorHandle::new(Arc::clone(&connection_manager), ingress_sender.clone(), egress_receiver);
    let actor = Self {
      ingress_receiver,
      ingress_sender,
      egress_sender,
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
      // println!("Processing message {:?}", message);
      match message {
        // As a host, accept a new client and create duplex connection
        ActorMessage::AcceptConnection {
          stream,
          host,
          new_client,
        } => {
          let new_connection_id = GenericConnectionId::new(
            *host.receiver(),
            self.connection_manager.get_channel_id(),
            SocketType::TCPClient,
          );
          self.connection_manager.set_client(&host.into(), new_connection_id.into());
          let new_connection = Connection::new(new_client, new_connection_id);
          self
            .egress_sender
            .send(NetEvent::NewClient {
              host_id: host,
              client: new_connection,
            })
            .ok();
          self.setup_duplex(stream, new_connection_id.clone());
        }
        // As a client, the host has connected and need to convert to duplex connection
        ActorMessage::FinalizeConnection { stream, connection } => {
          self.setup_duplex(stream, connection.id());
        }
        // Set up a new host
        ActorMessage::HostConnection(connection) => {
          self
            .connection_manager
            .set_channel_status(*connection.id().channel(), ConnectionStatus::Stable);
          tokio::spawn(Self::establish_connection_host(connection, self.ingress_sender.clone()));
        }
        // As a client, start requesting to connection to a host
        ActorMessage::ConnectTo(connection) => {
          tokio::spawn(Self::connect_to_host(connection, self.ingress_sender.clone()));
        }
        // Send a message over duplex comm (symmetric)
        ActorMessage::SendMessage(envelope) => {
          if &self.connection_senders.len() > envelope.connection_id.channel() {
            match self.connection_senders[*envelope.connection_id.channel()].as_ref() {
              Some(sender) => {
                sender.send(envelope).await.ok();
              }
              None => {}
            }
          }
        }
        // TODO: Proper cleanup
        ActorMessage::DropConnection(_) => {}
        ActorMessage::Shutdown => return Ok(()),
      };
    }
    Ok(())
  }
}

// Synchronous Helper methods
impl NetActor {
  fn setup_connection(&mut self, connection_id: GenericConnectionId) -> AsyncReceiver<Envelope> {
    while &self.connection_senders.len() <= connection_id.channel() {
      self.connection_senders.push(None);
    }
    let (tx, rx) = async_channel(128);
    self.connection_senders[*connection_id.channel()] = Some(tx);
    rx
  }

  fn setup_duplex(&mut self, stream: TcpStream, connection_id: GenericConnectionId) {
    let rx = self.setup_connection(connection_id);
    let (tcp_reader, tcp_writer) = stream.into_split();
    tokio::spawn(Self::establish_duplex_send(tcp_writer, rx));
    tokio::spawn(Self::establish_duplex_recv(
      tcp_reader,
      connection_id,
      self.egress_sender.clone(),
    ));
    self
      .connection_manager
      .set_channel_status(*connection_id.channel(), ConnectionStatus::Stable);
    self.egress_sender.send(NetEvent::Connected { connection_id }).ok();
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
    connection_id: GenericConnectionId,
    back_sender: Sender<NetEvent>,
  ) -> Result<(), Error> {
    loop {
      let mut size_bytes = 0u32.to_le_bytes();
      stream.read_exact(&mut size_bytes).await?;
      let packet_size = u32::from_le_bytes(size_bytes);
      let mut packet_buffer = Vec::with_capacity(packet_size as usize);
      for _ in 0..packet_size {
        packet_buffer.push(0);
      }
      stream.read_exact(&mut packet_buffer).await?;
      let response = Envelope::new(packet_buffer, connection_id);
      back_sender
        .send(NetEvent::Message {
          connection_id: connection_id,
          envelope: response,
        })
        .ok();
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
      ingress_sender
        .send(ActorMessage::AcceptConnection {
          stream: tcp_stream,
          new_client: ConnectionParameters::new(&path, port as u32),
          host: connection.id(),
        })
        .ok();
    }
    Ok(())
  }

  // TODO: Properly error handle
  // Acting as the client, request to connect to a remote host and wait for the connection to finish.
  async fn connect_to_host(connection: Connection, ingress_sender: UnboundedSender<ActorMessage>) -> Result<(), Error> {
    let tcp_stream = TcpStream::connect(connection.connection_string()).await?;
    ingress_sender
      .send(ActorMessage::FinalizeConnection {
        stream: tcp_stream,
        connection: connection.clone(),
      })
      .ok();
    Ok(())
  }
}
