use anyhow::Ok;
use godot::engine::{INode, Node};
use godot::prelude::*;
use proto::connect::Join;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpStream;
use tokio::runtime::{Builder, Runtime};

use crate::get_multi_single;

pub struct MultiPlayerConnection {}

impl MultiPlayerConnection {}

pub struct MultiManagerImpl {
    clients: HashMap<usize, MultiPlayerConnection>,
    socket: Option<TcpStream>,
    runtime: Runtime,
}

impl MultiManagerImpl {
    pub fn connect_to_server(&mut self, ip: String) -> anyhow::Result<()> {
        if self.socket.is_some() {
            godot_warn!("Socket has value,but reset")
        }
        self.socket = Some(TcpStream::from_std(std::net::TcpStream::connect(&ip)?)?);
        Ok(())
    }

    pub fn join_to_server(&mut self, player_name: String) -> anyhow::Result<()> {
        let mut data = Join {
            player_name,
            version: base::build::COMMIT_HASH.to_string(),
        };
        Ok(())
    }
}

impl MultiManagerImpl {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            socket: None,
            runtime: Builder::new_multi_thread().enable_all().build().unwrap(),
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
