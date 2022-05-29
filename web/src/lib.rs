mod socket;
mod state;
mod view;

use self::{
    socket::socket,
    state::{Channel, State},
    view::{App, Data, Event, Props},
};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use yew::AppHandle;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct View {
    app: AppHandle<App>,
}

impl View {
    fn received(&self, from: String, text: String) {
        self.app.send_message(Event::Received { from, text });
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use base::abi::{ClientMessage, ServerMessage};
    use gloo::console::log;
    use yew::Callback;

    let state = Rc::new(RefCell::new(State::default()));
    let (write, read) = socket("ws://127.0.0.1:4567");
    read.register({
        let state = Rc::clone(&state);
        move |message| match message {
            ServerMessage::Closed => log!("closed"),
            ServerMessage::LoggedIn(_) => log!("logged in"),
            ServerMessage::User(_) => log!("user"),
            ServerMessage::Channel(chan) => state
                .borrow_mut()
                .push_channel(Channel::new(&chan.name, chan.icon.as_deref())),
            ServerMessage::Said { .. } => log!("said"),
        }
    });

    write.request(ClientMessage::Login {
        name: "nano",
        pass: "123",
    });

    let root = gloo::utils::document()
        .get_element_by_id("root")
        .expect_throw("root");

    let app = yew::start_app_with_props_in_element::<App>(
        root,
        Props {
            data: Data {
                state,
                current_channel: 0,
                me: 0,
            },
            onaction: Callback::from(|_| todo!()),
        },
    );

    let view = View { app };
    view.received("nano".into(), "hi".into());

    Ok(())
}
