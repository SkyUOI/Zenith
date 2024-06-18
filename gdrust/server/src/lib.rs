mod cfg;

use anyhow::anyhow;
use bytes::BytesMut;
use cfg::{DEFAULT_IP, DEFAULT_PORT};
use clap::Parser;
use prost::Message;
use proto::connect::Join;
use std::{
    error::Error,
    io::{Cursor, Write},
    process::exit,
};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn read_join(&mut self) -> anyhow::Result<Option<Join>> {
        loop {
            if let Some(request) = self.parse_join()? {
                return Ok(Some(request));
            }
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    let msg = "Connection reset by peer";
                    log::info!("{}", msg);
                    return Err(anyhow!(msg));
                }
            }
        }
    }

    pub fn parse_join(&mut self) -> anyhow::Result<Option<Join>> {
        let buf = Cursor::new(&self.buffer[..]);
        match proto::connect::Join::decode(buf) {
            Ok(val) => Ok(Some(val)),
            Err(_) => Err(anyhow!("Not a join message")),
        }
    }
}

#[derive(Debug, Parser)]
#[command(
    author = "SkyUOI",
    version = crate::build::VERSION,
    about = "The Server Of Zenith",
    long_about = "Multi-player Game Server Of Zenith"
)]
#[command(propagate_version = true)]
struct ArgsParser {
    #[arg(short, long, default_value_t = DEFAULT_PORT)]
    port: i32,
    #[arg(long, default_value_t = String::from(DEFAULT_IP))]
    ip: String,
}

shadow_rs::shadow!(build);

async fn process_request(mut connect: Connection) -> anyhow::Result<()> {
    // 首先获取连接请求
    let join_data = connect.read_join().await?;
    loop {}
    Ok(())
}

pub async fn lib_main() -> Result<(), Box<dyn Error>> {
    let parser = ArgsParser::parse();
    let port = parser.port;
    let ip = parser.ip;
    let bind_addr = format!("{}:{}", ip, port);
    let tcplistener = match TcpListener::bind(&bind_addr).await {
        Ok(listener) => listener,
        Err(e) => {
            log::error!("Failed to bind {}:{}", bind_addr, e);
            exit(1);
        }
    };
    tokio::spawn(async move {
        loop {
            let ret = tcplistener.accept().await;
            match ret {
                Ok((socket, _)) => {
                    if let Err(e) = tokio::spawn(async {
                        if let Err(e) = process_request(Connection::new(socket)).await {
                            log::error!("When processing a request:{}", e)
                        }
                    })
                    .await
                    {
                        log::error!("Async error:{}", e);
                    }
                }
                Err(e) => {
                    log::error!("Accepting a new player failed:{}", e)
                }
            }
        }
    });
    loop {
        print!(">>>");
        std::io::stdout().flush().unwrap();
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;
        if command.trim() == "exit" {
            log::info!("Exiting now...");
            break;
        }
    }
    Ok(())
}
