use crate::{get_multi_single, get_tokio_runtime, MultiSingle};
use ::proto::ProtoRequest;
use anyhow::anyhow;
use bytes::{Bytes, BytesMut};
use godot::classes::{INode, Node};
use godot::prelude::*;
use prost::Message;
use proto::connect::Join;
use std::mem::swap;
use std::process::{Child, Command};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpStream;
use tokio::sync::mpsc;

pub struct MultiPlayerConnection {}

impl MultiPlayerConnection {}

pub struct MultiManagerImpl {
    /// 发送器
    socket: Option<mpsc::Sender<Bytes>>,
    /// 接收读取到的指令
    receiver: Option<std::sync::mpsc::Receiver<ProtoRequest>>,
    is_host: bool,
    server: Option<Child>,
}

pub type MessageReceiver = std::sync::mpsc::Receiver<ProtoRequest>;

async fn send(sender: mpsc::Sender<Bytes>, data: BytesMut) -> anyhow::Result<()> {
    sender.send(data.into()).await?;
    Ok(())
}

async fn write_loop(
    mut receiver: mpsc::Receiver<Bytes>,
    mut write_socket: OwnedWriteHalf,
) -> anyhow::Result<()> {
    godot_print!("Starting writing loop");
    while let Some(data) = receiver.recv().await {
        write_socket.write_all(&data).await?;
    }
    Ok(())
}

async fn read_loop(
    send_channel: std::sync::mpsc::Sender<ProtoRequest>,
    mut read_socket: OwnedReadHalf,
) -> anyhow::Result<()> {
    let mut buf = BytesMut::new();
    godot_print!("Starting reading loop");
    loop {
        let n = read_socket.read(&mut buf).await?;
        match parse_request(&buf) {
            None => {}
            Some(data) => send_channel.send(data)?,
        }
        if n == 0 {
            if buf.is_empty() {
                godot_print!("Connection closed");
                break;
            } else {
                let err_msg = "Connection reset by peer";
                godot_error!("{}", err_msg);
                return Err(anyhow::anyhow!(err_msg));
            }
        }
    }
    Ok(())
}

fn parse_request(buf: &BytesMut) -> Option<ProtoRequest> {
    match Join::decode(&buf[..]) {
        Ok(v) => Some(ProtoRequest::Join(v)),
        Err(_) => None,
    }
}

impl MultiManagerImpl {
    pub fn connect_to_server(&mut self, ip: String) -> anyhow::Result<()> {
        if self.socket.is_some() {
            godot_warn!("Socket has value,but reset")
        }
        let socket = std::net::TcpStream::connect(&ip)?;
        socket.set_nonblocking(true)?;
        let (sender, receiver) = mpsc::channel(32);
        let (request_sender, request_receiver) = std::sync::mpsc::channel();
        self.receiver = Some(request_receiver);
        self.socket = Some(sender);
        get_tokio_runtime().spawn(async move {
            let socket = TcpStream::from_std(socket)?;
            let (read_socket, write_socket) = socket.into_split();
            get_tokio_runtime().spawn(write_loop(receiver, write_socket));
            get_tokio_runtime().spawn(read_loop(request_sender, read_socket));
            anyhow::Ok(())
        });
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
        let mut buf = BytesMut::new();
        data.encode(&mut buf)?;
        let sender = socket.clone();
        get_tokio_runtime().spawn(send(sender, buf));
        Ok(())
    }

    pub fn close(&mut self) {
        self.socket = None;
    }

    pub fn send_data(&mut self, data: Bytes) {
        let socket = self.socket.clone();
        get_tokio_runtime().spawn(async {
            match socket {
                Some(socket) => {
                    socket.send(data).await?;
                }
                None => {
                    // Ignore in single mode
                }
            }
            anyhow::Ok(())
        });
    }

    pub fn alloc_id(&mut self) -> usize {
        let socket = self.socket.clone();
        // socket.unwrap().blocking_send(value);
        // get_tokio_runtime().spawn(async {
        //     match socket {
        //         Some(socket) => {}
        //         None => {}
        //     }
        // })
        todo!()
    }

    pub fn set_host(&mut self, val: bool) {
        self.is_host = val;
    }

    pub fn set_up_server(&mut self) {
        self.set_host(true);
        self.server = Some(
            Command::new("your_command")
                .spawn()
                .expect("Failed to start process"),
        );
    }

    pub fn borrow_receiver(&mut self) -> Option<std::sync::mpsc::Receiver<ProtoRequest>> {
        let mut tmp = None;
        swap(&mut tmp, &mut self.receiver);
        tmp
    }

    pub fn give_back_receiver(&mut self, receiver: std::sync::mpsc::Receiver<ProtoRequest>) {
        if self.receiver.is_some() {
            godot_error!("There is a receiver");
        }
        self.receiver = Some(receiver);
    }
}

impl MultiManagerImpl {
    pub fn new() -> Self {
        Self {
            socket: None,
            receiver: None,
            is_host: false,
            server: None,
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct MultiManager {
    base: Base<Node>,
}

#[godot_api()]
impl INode for MultiManager {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

#[godot_api()]
impl MultiManager {
    #[func]
    fn add_new_player(&mut self) {}
}

impl Drop for MultiManagerImpl {
    fn drop(&mut self) {
        match &mut self.server {
            None => {}
            Some(data) => {
                data.kill().expect("Stop server failed");
                data.wait().expect("Waited failed");
            }
        }
    }
}
