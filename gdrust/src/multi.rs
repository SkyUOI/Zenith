use anyhow::{anyhow, Ok};
use bytes::{Bytes, BytesMut};
use godot::engine::{INode, Node};
use godot::prelude::*;
use prost::Message;
use proto::connect::{self, Join};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use crate::{get_multi_single, get_tokio_runtime};

pub struct MultiPlayerConnection {}

impl MultiPlayerConnection {}

pub struct MultiManagerImpl {
    clients: HashMap<usize, MultiPlayerConnection>,
    socket: Option<mpsc::Sender<bytes::Bytes>>,
}

async fn send(sender: mpsc::Sender<Bytes>, data: BytesMut) -> anyhow::Result<()> {
    sender.send(data.into()).await?;
    Ok(())
}

impl MultiManagerImpl {
    pub fn connect_to_server(&mut self, ip: String) -> anyhow::Result<()> {
        if self.socket.is_some() {
            godot_warn!("Socket has value,but reset")
        }
        // self.socket = Some(Arc::new(Mutex::new(TcpStream::from_std(
        // std::net::TcpStream::connect(&ip)?,
        // )?)));
        let (sender, mut receiver) = mpsc::channel(32);
        self.socket = Some(sender);
        get_tokio_runtime().spawn(async move {
            let mut socket = TcpStream::connect(&ip).await?;
            while let Some(data) = receiver.recv().await {
                socket.write_all(&data).await?;
            }
            Ok(())
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
        }
    }
}

#[derive(GodotClass)]
#[class(base = Node)]
pub struct MultiManager {
    base: Base<Node>,
    multi_impl: Arc<Mutex<MultiManagerImpl>>,
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
