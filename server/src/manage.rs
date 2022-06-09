use crate::event::*;
use base::{api, decode, encode};
use rand::Rng;
use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
    sync::Arc,
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot::Sender as Close,
};

#[derive(Clone)]
struct User {
    id: u32,
    name: String,
    avatar: Option<String>,
}

struct Users {
    ids: HashMap<u32, Arc<User>>,
    names: HashMap<(String, String), Arc<User>>,
}

impl Users {
    fn new() -> Self {
        let mut users = Self {
            ids: HashMap::default(),
            names: HashMap::default(),
        };

        users.push_new("admin", "admin", None);
        users.push_new("test0", "test0", Some("./images/test0.jpg"));
        users.push_new("test1", "test1", Some("./images/test1.jpg"));
        users.push_new("test2", "test2", Some("./images/test2.jpg"));
        users.push_new("test3", "test3", Some("./images/test3.jpg"));
        users.push_new("test4", "test4", Some("./images/test4.jpg"));
        users
    }

    fn push_new(&mut self, name: &str, pass: &str, avatar: Option<&str>) -> Option<u32> {
        let name = name.to_owned();
        let pass = pass.to_owned();
        let id = self.ids.len() as u32;

        match self.names.entry((name.clone(), pass)) {
            Entry::Occupied(_) => None,
            Entry::Vacant(en) => {
                let user = Arc::new(User {
                    id,
                    name,
                    avatar: avatar.map(Into::into),
                });
                en.insert(Arc::clone(&user));
                self.ids.insert(id, user);
                Some(id)
            }
        }
    }

    fn get(&self, name: &str, pass: &str) -> Option<u32> {
        let key = (name.to_owned(), pass.to_owned());
        self.names.get(&key).map(|user| user.id)
    }

    fn get_by_id(&self, id: u32) -> Option<&User> {
        self.ids.get(&id).map(Arc::as_ref)
    }

    fn iter(&self) -> impl Iterator<Item = User> + '_ {
        self.ids.values().map(|user| user.as_ref().clone())
    }
}

#[derive(Clone)]
struct Channel {
    id: u32,
    name: String,
    icon: Option<String>,
}

struct Channels(Vec<Channel>);

impl Channels {
    fn new() -> Self {
        Self(vec![
            Channel {
                id: 0,
                name: "Общение".into(),
                icon: Some("./images/chatting.png".into()),
            },
            Channel {
                id: 1,
                name: "Разработка".into(),
                icon: Some("./images/development.png".into()),
            },
            Channel {
                id: 2,
                name: "Программирование".into(),
                icon: Some("./images/code.png".into()),
            },
            Channel {
                id: 3,
                name: "Игры".into(),
                icon: Some("./images/games.png".into()),
            },
        ])
    }

    fn iter(&self) -> impl Iterator<Item = Channel> + '_ {
        self.0.iter().cloned()
    }
}

pub async fn manage(mut receiver: Receiver<Event>) -> ! {
    use api::*;

    struct Client {
        sender: Sender<Vec<u8>>,
        close: Option<Close<()>>,
        logged: Option<u32>,
    }

    let mut users = Users::new();
    let channels = Channels::new();
    let mut clients: HashMap<SocketAddr, Client> = HashMap::default();
    let mut history = vec![];

    loop {
        let event = receiver.recv().await.expect("channel is open");

        match event.what {
            What::NewConnection { sender, close } => {
                // Remove old client
                let _ = clients.insert(
                    event.from,
                    Client {
                        sender,
                        close: Some(close),
                        logged: None,
                    },
                );
            }
            What::CloseConnection => {
                let _ = clients.remove(&event.from);
            }
            What::BytesReceived(bytes) => {
                let client = clients.get_mut(&event.from).expect("client");

                let message = match decode(&bytes) {
                    Ok(message) => match message {
                        ClientMessage::SignUp { name, pass } => ServerMessage::LoggedIn(
                            users
                                .push_new(name, pass, None)
                                .ok_or(LoginError::NameAlreadyExists),
                        ),
                        ClientMessage::Login { name, pass } => {
                            let logged = match users.get(name, pass) {
                                Some(id) => match client.logged {
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
                        ClientMessage::Say { chan, text } => match client.logged {
                            Some(id) => {
                                let user = users.get_by_id(id).expect("user");
                                let name = &user.name;
                                println!("{name} ({chan}): {text}");

                                let message = Message {
                                    from: id,
                                    chan,
                                    content: MessageType::Text(text.into()),
                                };

                                // Send this to all clients
                                for client in clients.values() {
                                    let message = ServerMessage::Message(message.clone());
                                    send(&client.sender, message).await;
                                }

                                history.push(message);
                                continue;
                            }
                            None => ServerMessage::Closed,
                        },
                        ClientMessage::File { chan, ext, bytes } => match client.logged {
                            Some(id) => {
                                let saved = save_file(ext, bytes);
                                println!("saved file {}", saved);

                                let message = Message {
                                    from: id,
                                    chan,
                                    content: MessageType::File(saved),
                                };

                                // Send this to all clients
                                for client in clients.values() {
                                    let message = ServerMessage::Message(message.clone());
                                    send(&client.sender, message).await;
                                }

                                history.push(message);
                                continue;
                            }
                            None => ServerMessage::Closed,
                        },
                    },
                    Err(err) => {
                        println!("{}: decode error {:?}", event.from, err);
                        ServerMessage::Closed
                    }
                };

                if let ServerMessage::Closed = message {
                    client.close.take().map(|close| close.send(()));
                }

                let send_initial_data = matches!(message, ServerMessage::LoggedIn(Ok(_)));
                let sender = &client.sender;
                send(sender, message).await;

                if send_initial_data {
                    for user in users.iter() {
                        let message = ServerMessage::User(User {
                            id: user.id,
                            name: user.name,
                            avatar: user.avatar,
                        });
                        send(sender, message).await;
                    }

                    for chan in channels.iter() {
                        let message = ServerMessage::Channel(Channel {
                            id: chan.id,
                            name: chan.name,
                            icon: chan.icon,
                            history: history
                                .iter()
                                .filter(|message| message.chan == chan.id)
                                .cloned()
                                .collect(),
                        });
                        send(sender, message).await;
                    }
                }
            }
        }
    }
}

async fn send(sender: &Sender<Vec<u8>>, message: api::ServerMessage) {
    let mut buf = Vec::with_capacity(64);
    encode(&message, &mut buf).expect("encode");
    let _ = sender.send(buf).await;
}

fn save_file(ext: &str, bytes: &[u8]) -> String {
    use std::{fs::File, io::Write, path::PathBuf};

    let mut rng = rand::thread_rng();
    let name = {
        let mut name: String = (0..20)
            .map(|_| match rng.gen_range(0..=2) {
                0 => rng.gen_range('a'..='z'),
                1 => rng.gen_range('A'..='Z'),
                2 => rng.gen_range('0'..='9'),
                _ => unreachable!(),
            })
            .collect();

        name.push('.');
        name.push_str(ext);
        name
    };

    let mut path = PathBuf::from("./static/images");
    path.push(&name);
    let mut file = File::create(&path).expect("create file");
    file.write_all(bytes).expect("write");
    file.flush().expect("flush");

    name
}
