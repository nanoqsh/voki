use crate::event::*;
use base::{abi, decode, encode};
use std::{
    collections::{hash_map::Entry, HashMap},
    net::SocketAddr,
    rc::Rc,
};
use tokio::sync::mpsc::{Receiver, Sender};

#[derive(Clone)]
struct User {
    id: u32,
    name: String,
    avatar: Option<String>,
}

#[derive(Default)]
struct Users {
    ids: HashMap<u32, Rc<User>>,
    names: HashMap<(String, String), Rc<User>>,
}

impl Users {
    fn push_new(&mut self, name: &str, pass: &str) -> Option<u32> {
        let name = name.to_owned();
        let pass = pass.to_owned();
        let id = self.ids.len() as u32;

        match self.names.entry((name.clone(), pass)) {
            Entry::Occupied(_) => None,
            Entry::Vacant(en) => {
                let user = Rc::new(User {
                    id,
                    name,
                    avatar: None,
                });
                en.insert(Rc::clone(&user));
                self.ids.insert(id, user);
                Some(id)
            }
        }
    }

    fn get(&self, name: &str, pass: &str) -> Option<u32> {
        let key = (name.to_owned(), pass.to_owned());
        self.names.get(&key).map(|user| user.id)
    }

    fn iter(&self) -> impl Iterator<Item = User> + '_ {
        self.ids.values().map(|user| user.as_ref().clone())
    }
}

pub async fn manage(mut receiver: Receiver<Event>) -> ! {
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

    struct Client {
        sender: Sender<Vec<u8>>,
        logged: Option<u32>,
    }

    let channels = Channel::channels();
    let mut users = Users::default();
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
                        ClientMessage::SignUp { name, pass } => ServerMessage::LoggedIn(
                            users
                                .push_new(name, pass)
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
                    for user in users.iter() {
                        send(
                            &client.sender,
                            ServerMessage::User(abi::User {
                                id: user.id,
                                name: user.name,
                                avatar: user.avatar,
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
