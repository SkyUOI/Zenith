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
    io::{self, AsyncBufReadExt, AsyncReadExt, BufReader},
    net::{TcpListener, TcpStream},
    select,
    sync::broadcast,
};

struct Connection {
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

    pub async fn read_join(&mut self) -> anyhow::Result<Option<Join>> {
        loop {
            if self.shutdown.try_recv().is_ok() {
                log::info!("Exit");
                return Err(anyhow!("Exit"));
            }
            let sz = self.stream.read_buf(&mut self.buffer).await?;
            log::info!("Received:{}", sz);
            if 0 == sz {
                if self.buffer.is_empty() {
                    log::info!("Join exiting(empty)...");
                    return Ok(None);
                } else {
                    let msg = "Connection reset by peer";
                    log::info!("{}", msg);
                    return Err(anyhow!(msg));
                }
            }
            if let Some(request) = self.parse_join()? {
                log::info!("Join exiting...");
                return Ok(Some(request));
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
    version = base::build::VERSION,
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

async fn process_request(mut connect: Connection) -> anyhow::Result<()> {
    // 首先获取连接请求
    log::info!("start joining");
    let join_data = connect.read_join().await?;
    log::info!("joined");
    let request = async { loop {} };
    select! {
        _ = request => {},
        _ = connect.shutdown.recv() => {
            log::info!("Player exited")
        }
    }
    Ok(())
}

async fn accept_sockets(
    tcplistener: TcpListener,
    shutdown_sender: broadcast::Sender<()>,
    mut shutdown_receiver: broadcast::Receiver<()>,
) {
    let async_loop = async move {
        loop {
            let ret = tcplistener.accept().await;
            match ret {
                Ok((socket, _)) => {
                    let shutdown = shutdown_sender.subscribe();
                    log::info!("Connected to a socket");
                    tokio::spawn(async move {
                        if let Err(e) = process_request(Connection::new(shutdown, socket)).await {
                            log::error!("When processing a request:{}", e)
                        }
                        log::info!("A socket exited successful");
                    });
                }
                Err(e) => {
                    log::error!("Accepting a new player failed:{}", e)
                }
            }
        }
    };
    select! {
        _ = async_loop => {}
        _ = shutdown_receiver.recv() => {
                log::info!("Accepting loop exited")
        }
    }
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
    let (shutdown_sender, mut shutdown_receiver) = broadcast::channel(32);
    tokio::spawn(accept_sockets(
        tcplistener,
        shutdown_sender.clone(),
        shutdown_sender.subscribe(),
    ));
    tokio::spawn(async move {
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                log::info!("Exiting now...");
                shutdown_sender.send(())?;
            }
            Err(err) => {
                log::error!("Unable to listen for shutdown signal: {}", err);
                shutdown_sender.send(())?;
            }
        }
        anyhow::Ok(())
    });
    let mut console_reader = BufReader::new(io::stdin()).lines();
    let input_loop = async {
        loop {
            print!(">>>");
            std::io::stdout().flush().unwrap();
            let command = match console_reader.next_line().await {
                Ok(d) => match d {
                    Some(data) => data,
                    None => {
                        break;
                    }
                },
                Err(e) => {
                    log::error!("{}", e);
                    break;
                }
            };
            if command.trim() == "exit" {
                log::info!("Exiting now...");
                break;
            }
        }
        anyhow::Ok(())
    };
    select! {
        _ = input_loop => {},
        _ = shutdown_receiver.recv() => {
            log::info!("Command loop exited")
        }
    }
    Ok(())
}
