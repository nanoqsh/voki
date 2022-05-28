mod args;
mod event;

use self::{args::Args, event::*};
use base::{abi, decode, encode};
use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender},
};
use websocket::tungstenite::{self as ws, Message};

#[tokio::main]
async fn main() {
    use clap::Parser;

    let args = Args::parse();

    let (sender, receiver) = mpsc::channel(16);
    tokio::select! {
        _ = listen(args.address(), sender) => {},
        _ = manage(receiver) => {},
    }
}

async fn listen(addr: &str, sender: Sender<Event>) -> ! {
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

                        let _ = sender.send(event);
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

async fn manage(mut receiver: Receiver<Event>) -> ! {
    use abi::*;

    struct Channel {
        name: String,
        icon: Option<String>,
    }

    impl Channel {
        fn channels() -> Vec<Self> {
            vec![
                Self {
                    name: "Chatting".into(),
                    icon: None,
                },
                Self {
                    name: "Coding".into(),
                    icon: None,
                },
                Self {
                    name: "Games".into(),
                    icon: None,
                },
            ]
        }
    }

    #[derive(Hash, PartialEq, Eq)]
    struct User {
        name: String,
        pass: String,
    }

    struct UserData {
        id: u32,
        avatar: Option<String>,
    }

    struct Client {
        sender: Sender<Vec<u8>>,
        logged: Option<u32>,
    }

    let channels = Channel::channels();
    let mut users: HashMap<User, UserData> = HashMap::default();
    let mut clients: HashMap<SocketAddr, Client> = HashMap::default();

    loop {
        let event = receiver.recv().await.expect("channel is open");

        match event.what {
            What::NewConnection(sender) => {
                // Remove old client
                let _ = clients.insert(
                    event.from,
                    Client {
                        sender,
                        logged: None,
                    },
                );
            }
            What::CloseConnection => {
                let old = clients.remove(&event.from);
                assert!(old.is_some());
            }
            What::BytesReceived(bytes) => {
                let client = clients.get_mut(&event.from).expect("client");

                let message = match decode(&bytes) {
                    Ok(message) => match message {
                        ClientMessage::SignUp { name, pass } => {
                            let id = users.len() as u32;

                            let logged = match users.entry(User {
                                name: name.into(),
                                pass: pass.into(),
                            }) {
                                Entry::Occupied(_) => Err(LoginError::NameAlreadyExists),
                                Entry::Vacant(en) => {
                                    en.insert(UserData { id, avatar: None });
                                    Ok(id)
                                }
                            };

                            ServerMessage::LoggedIn(logged)
                        }
                        ClientMessage::Login { name, pass } => {
                            let user = User {
                                name: name.into(),
                                pass: pass.into(),
                            };

                            let logged = match users.get(&user) {
                                Some(&UserData { id, .. }) => match client.logged {
                                    Some(_) => Err(LoginError::AlreadyLogged),
                                    None => {
                                        client.logged = Some(id);
                                        Ok(id)
                                    }
                                },
                                None => Err(LoginError::WrongNameOrPass),
                            };

                            ServerMessage::LoggedIn(logged)
                        }
                        ClientMessage::Say { text } => match client.logged {
                            Some(id) => ServerMessage::Said { from: id, text },
                            None => ServerMessage::Closed,
                        },
                    },
                    Err(err) => {
                        println!("{}: decode error {:?}", event.from, err);
                        // TODO: Close connection
                        ServerMessage::Closed
                    }
                };

                let send_initial_data = matches!(message, ServerMessage::LoggedIn(_));
                send(&client.sender, message).await;

                if send_initial_data {
                    for (user, data) in &users {
                        send(
                            &client.sender,
                            ServerMessage::User(abi::User {
                                id: data.id,
                                name: user.name.clone(),
                                avatar: data.avatar.clone(),
                            }),
                        )
                        .await;
                    }

                    for (id, chan) in channels.iter().enumerate() {
                        send(
                            &client.sender,
                            ServerMessage::Channel(abi::Channel {
                                id: id as u32,
                                name: chan.name.clone(),
                                icon: chan.icon.clone(),
                            }),
                        )
                        .await;
                    }
                }
            }
        }
    }
}

async fn send(sender: &Sender<Vec<u8>>, message: abi::ServerMessage<'_>) {
    let mut buf = Vec::with_capacity(64);
    encode(&message, &mut buf).expect("encode");
    let _ = sender.send(buf).await;
}
