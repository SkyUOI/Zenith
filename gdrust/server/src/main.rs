mod cfg;

use bytes::BytesMut;
use cfg::{DEFAULT_IP, DEFAULT_PORT};
use clap::Parser;
use std::error::Error;
use tokio::net::{TcpListener, TcpStream};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let parser = ArgsParser::parse();
    let port = parser.port;
    let ip = parser.ip;
    let bind_ip = format!("{}:{}", ip, port);
    let tcplistener = TcpListener::bind(&bind_ip).await?;
    loop {
        let ret = tcplistener.accept().await;
        match ret {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    Ok(())
}
