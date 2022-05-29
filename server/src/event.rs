use std::net::SocketAddr;
use tokio::sync::{mpsc::Sender, oneshot::Sender as Close};

pub enum What {
    NewConnection {
        sender: Sender<Vec<u8>>,
        close: Close<()>,
    },
    CloseConnection,
    BytesReceived(Vec<u8>),
}

pub struct Event {
    pub from: SocketAddr,
    pub what: What,
}
