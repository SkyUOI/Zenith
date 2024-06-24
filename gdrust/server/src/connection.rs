use std::io::Cursor;

use anyhow::anyhow;
use bytes::BytesMut;
use prost::Message;
use proto::{connect, ProtoRequest};
use tokio::{io::AsyncReadExt, net::TcpStream, sync::broadcast};

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    shutdown: broadcast::Receiver<()>,
}

impl Connection {
    pub fn new(shutdown: broadcast::Receiver<()>, stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(1024),
            shutdown,
        }
    }

    pub async fn read_join(&mut self) -> anyhow::Result<Option<connect::Join>> {
        loop {
            if self.shutdown.try_recv().is_ok() {
                log::info!("Exit");
                return Err(anyhow!("Exit"));
            }
            if let Some(request) = self.parse_join()? {
                log::info!("Join exiting...");
                return Ok(Some(request));
            }
            let sz = self.stream.read_buf(&mut self.buffer).await?;
            log::info!("Received:{}", sz);
            if 0 == sz {
                return if self.buffer.is_empty() {
                    log::info!("Join exiting(empty)...");
                    Ok(None)
                } else {
                    let msg = "Connection reset by peer";
                    log::info!("{}", msg);
                    Err(anyhow!(msg))
                };
            }
        }
    }

    pub fn parse_join(&mut self) -> anyhow::Result<Option<connect::Join>> {
        let buf = Cursor::new(&self.buffer[..]);
        match connect::Join::decode(buf) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Err(anyhow!("Not a join message")),
        }
    }

    pub async fn process_request(&mut self) -> anyhow::Result<()> {
        loop {
            if self.shutdown.try_recv().is_ok() {
                return Err(anyhow!("Exit"));
            }
            if let Some(request) = self.parse_request()? {}
            let sz = self.stream.read_buf(&mut self.buffer).await?;
            if 0 == sz {
                return if self.buffer.is_empty() {
                    Ok(())
                } else {
                    let msg = "Connection reset by peer";
                    log::info!("{}", msg);
                    Err(anyhow!(msg))
                };
            }
        }
    }

    pub fn parse_request(&mut self) -> anyhow::Result<Option<ProtoRequest>> {
        let buf = Cursor::new(&self.buffer[..]);
        match proto::connect::CreateObj::decode(buf) {
            Ok(data) => Ok(Some(ProtoRequest::CreateObj(data))),
            Err(_) => Err(anyhow!("Not a message")),
        }
    }

    pub async fn start(&mut self) -> anyhow::Result<()> {
        // 首先获取连接请求
        log::info!("start joining");
        let join_data = self.read_join().await?;
        log::info!("joined");
        self.process_request().await?;
        Ok(())
    }
}
