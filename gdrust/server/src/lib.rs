mod cfg;

use anyhow::anyhow;
use bytes::BytesMut;
use cfg::{DEFAULT_IP, DEFAULT_PORT};
use clap::Parser;
use prost::Message;
use proto::{connect::Join, ProtoRequest};
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

struct Server {
    alloc_id: usize,
    ip: String,
    port: usize,
    bind_addr: String,
    tcplistener: TcpListener,
}

impl Server {
    pub async fn new(ip: impl Into<String>, port: usize) -> anyhow::Result<Self> {
        let ip = ip.into();
        let bind_addr = format!("{}:{}", ip.clone(), port);
        let tcplistener = match TcpListener::bind(&bind_addr).await {
            Ok(listener) => listener,
            Err(e) => {
                log::error!("Failed to bind {}:{}", bind_addr, e);
                exit(1);
            }
        };
        Ok(Self {
            alloc_id: 0,
            ip: ip.clone(),
            port,
            bind_addr,
            tcplistener,
        })
    }

    async fn accept_sockets(
        &mut self,
        shutdown_sender: broadcast::Sender<()>,
        mut shutdown_receiver: broadcast::Receiver<()>,
    ) {
        let async_loop = async move {
            loop {
                let ret = self.tcplistener.accept().await;
                match ret {
                    Ok((socket, _)) => {
                        let shutdown = shutdown_sender.subscribe();
                        log::info!("Connected to a socket");
                        tokio::spawn(async move {
                            if let Err(e) = process_request(Connection::new(shutdown, socket)).await
                            {
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

    pub fn parse_join(&mut self) -> anyhow::Result<Option<Join>> {
        let buf = Cursor::new(&self.buffer[..]);
        match Join::decode(buf) {
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
    port: usize,
    #[arg(long, default_value_t = String::from(DEFAULT_IP))]
    ip: String,
}

async fn process_request(mut connect: Connection) -> anyhow::Result<()> {
    // 首先获取连接请求
    log::info!("start joining");
    let join_data = connect.read_join().await?;
    log::info!("joined");
    connect.process_request().await?;
    Ok(())
}

pub async fn lib_main() -> Result<(), Box<dyn Error>> {
    let parser = ArgsParser::parse();
    let port = parser.port;
    let ip = parser.ip;
    let (shutdown_sender, mut shutdown_receiver) = broadcast::channel(32);
    let mut server = Server::new(ip, port).await?;
    let shutdown_sender_clone = shutdown_sender.clone();
    let shutdown_receiver_clone = shutdown_sender.subscribe();
    tokio::spawn(async move {
        server
            .accept_sockets(shutdown_sender_clone, shutdown_receiver_clone)
            .await
    });
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
