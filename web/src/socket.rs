use crate::post::{post, Receiver, Sender};
use base::{api, decode, encode};
use futures::{SinkExt, StreamExt};
use gloo::{
    console::error,
    net::websocket::{futures::WebSocket, Message, WebSocketError},
    timers::future,
};
use std::time::Duration;

pub struct Write(Sender<Message>);

impl Write {
    pub fn request(&self, message: api::ClientMessage) {
        let mut buf = Vec::with_capacity(64);
        encode(&message, &mut buf).expect("encode");
        self.0.push(Message::Bytes(buf));
    }
}

pub struct Read(Receiver<Message>);

impl Read {
    pub fn register<F>(self, mut callback: F)
    where
        F: FnMut(api::ServerMessage) + 'static,
    {
        wasm_futures::spawn_local(async move {
            loop {
                while let Some(Message::Bytes(bytes)) = self.0.take() {
                    let message = decode(&bytes).expect("decode");
                    callback(message);
                }

                future::sleep(Duration::from_millis(1)).await;
            }
        });
    }
}

pub fn socket(url: &str) -> (Write, Read) {
    let ws = WebSocket::open(url).expect("websocket opened");
    let (mut write, mut read) = ws.split();
    let (write_sender, write_receiver) = post();
    let (read_sender, read_receiver) = post();

    wasm_futures::spawn_local(async move {
        loop {
            for message in &write_receiver {
                let _ = write.send(message).await;
            }

            future::sleep(Duration::from_millis(1)).await;
        }
    });

    wasm_futures::spawn_local(async move {
        while let Some(res) = read.next().await {
            match res {
                Ok(message) => read_sender.push(message),
                Err(err) => match err {
                    WebSocketError::ConnectionError | WebSocketError::ConnectionClose(_) => break,
                    WebSocketError::MessageSendError(err) => {
                        error!("{}: {}", err.name, err.message)
                    }
                    _ => continue,
                },
            }
        }
    });

    (Write(write_sender), Read(read_receiver))
}
