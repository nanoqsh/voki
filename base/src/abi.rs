use bincode::{BorrowDecode, Decode, Encode};

#[derive(BorrowDecode, Encode)]
pub enum ClientMessage<'a> {
    SignUp { name: &'a str, pass: &'a str },
    Login { name: &'a str, pass: &'a str },
    Say { text: &'a str },
}

#[derive(Decode, Encode)]
pub enum LoginError {
    NameAlreadyExists,
    AlreadyLogged,
    WrongNameOrPass,
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
}

#[derive(BorrowDecode, Encode)]
pub enum ServerMessage<'a> {
    Closed,
    LoggedIn(Result<u32, LoginError>),
    User(User),
    Channel(Channel),
    Said { from: u32, text: &'a str },
}
