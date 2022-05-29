use crate::event::*;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Sender},
};
use websocket::tungstenite::{self as ws, Message};

pub async fn listen(addr: String, sender: Sender<Event>) -> ! {
    let listener = TcpListener::bind(addr).await.expect("bind");
    let local_addr = listener.local_addr().expect("should have a local adders");
    println!("listening at {local_addr}");

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let connect_sender = sender.clone();
                let close_sender = sender.clone();

                tokio::spawn(async move {
                    if let Err(err) = connect(stream, connect_sender).await {
                        eprintln!("websocket error: {err:?}");
                    }

                    // Better treat with it via guard
                    let event = Event {
                        from: addr,
                        what: What::CloseConnection,
                    };
                    let _ = close_sender.send(event).await;

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
    let _ = sender.send(event).await;

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

                        let _ = sender.send(event).await;
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
