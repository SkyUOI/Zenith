mod cfg;

use anyhow::anyhow;
use bytes::BytesMut;
use cfg::{DEFAULT_IP, DEFAULT_PORT};
use clap::Parser;
use std::{
    error::Error,
    io::{Cursor, Write},
    process::exit,
    thread,
};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
};

struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

#[derive(Debug)]
/// 请求类型
enum Request {
    /// 加入服务器的请求
    Join,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(1024),
        }
    }

    pub async fn read_request(&mut self) -> anyhow::Result<Option<Request>> {
        loop {
            if let Some(request) = self.parse_request()? {
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

    pub fn parse_request(&mut self) -> anyhow::Result<Option<Request>> {
        let mut buf = Cursor::new(&self.buffer[..]);

        todo!()
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

async fn process_request(connect: Connection) {
    loop {}
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
                    if let Err(e) =
                        tokio::spawn(async { process_request(Connection::new(socket)).await }).await
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
