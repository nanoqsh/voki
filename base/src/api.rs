use bincode::{BorrowDecode, Decode, Encode};
use std::fmt;

#[derive(BorrowDecode, Encode)]
pub enum ClientMessage<'a> {
    SignUp {
        name: &'a str,
        pass: &'a str,
    },
    Login {
        name: &'a str,
        pass: &'a str,
    },
    Say {
        chan: u32,
        text: &'a str,
    },
    File {
        chan: u32,
        ext: &'a str,
        bytes: &'a [u8],
    },
}

#[derive(Decode, Encode)]
pub enum LoginError {
    NameAlreadyExists,
    AlreadyLogged,
    WrongNameOrPass,
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NameAlreadyExists => write!(f, "name already exists"),
            Self::AlreadyLogged => write!(f, "alreadyL logged"),
            Self::WrongNameOrPass => write!(f, "wrong name or pass"),
        }
    }
}

#[derive(Decode, Encode)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub avatar: Option<String>,
}

#[derive(Decode, Encode)]
pub struct Channel {
    pub id: u32,
    pub name: String,
    pub icon: Option<String>,
    pub history: Vec<Message>,
}

#[derive(Clone, Decode, Encode)]
pub enum MessageType {
    Text(String),
    File(String),
}

#[derive(Clone, Decode, Encode)]
pub struct Message {
    pub from: u32,
    pub chan: u32,
    pub content: MessageType,
}

#[derive(Decode, Encode)]
pub enum ServerMessage {
    Closed,
    LoggedIn(Result<u32, LoginError>),
    User(User),
    Channel(Channel),
    Message(Message),
}
