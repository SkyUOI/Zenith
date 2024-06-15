mod cfg;

use bytes::BytesMut;
use cfg::{DEFAULT_IP, DEFAULT_PORT};
use clap::Parser;
use std::{error::Error, io::Write, process::exit, thread};
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

async fn process_request(connect: Connection) {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("Server initing...");
    let parser = ArgsParser::parse();
    let port = parser.port;
    let ip = parser.ip;
    let bind_ip = format!("{}:{}", ip, port);
    let tcplistener = match TcpListener::bind(&bind_ip).await {
        Ok(listener) => listener,
        Err(e) => {
            log::error!("Failed to bind {}:{}", bind_ip, e);
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
