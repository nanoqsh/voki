mod args;

use self::args::Args;
use std::net::SocketAddr;
use tokio::{
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::mpsc::{self, Sender},
};
use websocket::tungstenite::{self as ws, Message};

pub enum What {
    NewConnection(Sender<Vec<u8>>),
    BytesReceived(Vec<u8>),
}

pub struct Event {
    pub from: SocketAddr,
    pub what: What,
}

#[tokio::main]
async fn main() {
    use clap::Parser;

    let args = Args::parse();

    let (sender, _receiver) = mpsc::channel(16);
    run(args.address(), sender).await;
}

async fn run<A>(addr: A, sender: Sender<Event>) -> !
where
    A: ToSocketAddrs,
{
    let listener = TcpListener::bind(addr).await.expect("bind");
    let local_addr = listener.local_addr().expect("should have a local adders");
    println!("listening at {local_addr}");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let sender = sender.clone();
                tokio::spawn(async move {
                    if let Err(err) = connect(stream, sender).await {
                        eprintln!("websocket error: {err:?}");
                    }

                    println!("connection closed {addr}");
                });
            }
            Err(err) => {
                eprintln!("couldn't get client: {err:?}");
                continue;
            }
        }
    }
}

async fn connect(stream: TcpStream, sender: Sender<Event>) -> Result<(), ws::Error> {
    use futures::{SinkExt, StreamExt};

    let addr = stream.peer_addr().expect("peer address");
    let stream = websocket::accept_async(stream).await?;
    println!("new websocket client: {addr}");

    let (client_sender, mut receiver) = mpsc::channel(16);
    let event = Event {
        from: addr,
        what: What::NewConnection(client_sender),
    };
    if sender.send(event).await.is_err() {
        return Ok(());
    }

    let (mut write, mut read) = stream.split();
    loop {
        tokio::select! {
            Some(res) = read.next() => {
                match res? {
                    Message::Binary(bytes) => {
                        let event = Event {
                            from: addr,
                            what: What::BytesReceived(bytes),
                        };

                        if sender.send(event).await.is_err() {
                            return Ok(());
                        }
                    }
                    Message::Ping(bytes) => write.send(Message::Pong(bytes)).await?,
                    Message::Close(_) => return Ok(()),
                    _ => {}
                }
            }
            Some(bytes) = receiver.recv() => write.send(Message::Binary(bytes)).await?,
        }
    }
}
