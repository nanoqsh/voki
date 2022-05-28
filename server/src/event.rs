use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

pub enum What {
    NewConnection(Sender<Vec<u8>>),
    CloseConnection,
    BytesReceived(Vec<u8>),
}

pub struct Event {
    pub from: SocketAddr,
    pub what: What,
}
