mod cfg;
mod connection;

use cfg::{DEFAULT_IP, DEFAULT_PORT};
use clap::Parser;
use connection::Connection;
use std::{collections::HashMap, error::Error, io::Write, process::exit};
use tokio::{
    io::{self, AsyncBufReadExt, BufReader},
    net::TcpListener,
    select,
    sync::broadcast,
};

struct Server {
    alloc_id: usize,
    ip: String,
    port: usize,
    bind_addr: String,
    tcplistener: TcpListener,
    connections: HashMap<usize, Connection>,
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
            connections: HashMap::new(),
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
                        tokio::spawn(async {
                            let mut connection = Connection::new(shutdown, socket);
                            if let Err(e) = connection.start().await {
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
