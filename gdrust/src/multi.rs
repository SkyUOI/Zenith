use anyhow::anyhow;
use bytes::{Bytes, BytesMut};
use godot::engine::{INode, Node};
use godot::prelude::*;
use prost::Message;
use proto::connect::Join;
use proto::proto;
use std::collections::HashMap;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::{get_multi_single, get_tokio_runtime, MultiSingle};

pub struct MultiPlayerConnection {}

impl MultiPlayerConnection {}

enum Requests {
    Join(Join),
    Wrong(String),
    ExitSuccess,
}

pub struct MultiManagerImpl {
    clients: HashMap<usize, MultiPlayerConnection>,
    socket: Option<mpsc::Sender<bytes::Bytes>>,
    receiver: Option<std::sync::mpsc::Receiver<Requests>>,
}

async fn send(sender: mpsc::Sender<Bytes>, data: BytesMut) -> anyhow::Result<()> {
    sender.send(data.into()).await?;
    Ok(())
}

async fn write_loop(
    mut receiver: mpsc::Receiver<Bytes>,
    mut write_socket: OwnedWriteHalf,
) -> anyhow::Result<()> {
    while let Some(data) = receiver.recv().await {
        write_socket.write_all(&data).await?;
    }
    Ok(())
}

async fn read_loop(
    send_channel: std::sync::mpsc::Sender<Requests>,
    mut read_socket: OwnedReadHalf,
) -> anyhow::Result<()> {
    let mut buf = BytesMut::new();
    loop {
        let n = read_socket.read(&mut buf).await?;
        match parse_request(&buf) {
            None => {}
            Some(data) => send_channel.send(data)?,
        }
        if n == 0 {
            if buf.is_empty() {
                godot_print!("Connection closed");
                send_channel.send(Requests::ExitSuccess)?;
                break;
            } else {
                let err_msg = "Connection reset by peer";
                godot_error!("{}", err_msg);
                send_channel.send(Requests::Wrong(err_msg.to_string()))?;
                return Err(anyhow::anyhow!(err_msg));
            }
        }
    }
    Ok(())
}

fn parse_request(buf: &BytesMut) -> Option<Requests> {
    match proto::connect::Join::decode(&buf[..]) {
        Ok(v) => Some(Requests::Join(v)),
        Err(_) => None,
    }
}

impl MultiManagerImpl {
    pub fn connect_to_server(&mut self, ip: String) -> anyhow::Result<()> {
        if self.socket.is_some() {
            godot_warn!("Socket has value,but reset")
        }
        let socket = std::net::TcpStream::connect(&ip)?;
        let socket = TcpStream::from_std(socket)?;
        let (read_socket, write_socket) = socket.into_split();
        let (sender, receiver) = mpsc::channel(32);
        let (request_sender, request_receiver) = std::sync::mpsc::channel();
        self.receiver = Some(request_receiver);
        self.socket = Some(sender);
        get_tokio_runtime().spawn(write_loop(receiver, write_socket));
        get_tokio_runtime().spawn(read_loop(request_sender, read_socket));
        Ok(())
    }

    pub fn join_to_server(&mut self, player_name: String) -> anyhow::Result<()> {
        let socket = match &mut self.socket {
            None => {
                godot_error!("Socket doesn't exist");
                return Err(anyhow!("Socket doesn't exist"));
            }
            Some(s) => s,
        };
        let data = Join {
            player_name,
            version: base::build::COMMIT_HASH.to_string(),
        };
        let mut buf = bytes::BytesMut::new();
        data.encode(&mut buf)?;
        let sender = socket.clone();
        get_tokio_runtime().spawn(send(sender, buf));
        Ok(())
    }

    pub fn close(&mut self) {
        self.socket = None;
    }
}

impl MultiManagerImpl {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            socket: None,
            receiver: None,
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct MultiManager {
    base: Base<Node>,
    multi_impl: MultiSingle,
}

#[godot_api()]
impl INode for MultiManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            multi_impl: get_multi_single().clone(),
        }
    }
}

#[godot_api()]
impl MultiManager {
    #[func]
    fn add_new_player(&mut self) {}
}
