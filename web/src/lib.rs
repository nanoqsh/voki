mod post;
mod socket;
mod state;
mod view;

use self::{
    socket::socket,
    state::{Channel, Message, State},
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
    fn update(&self) {
        self.app.send_message(Event::StateUpdated);
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use base::abi::{ClientMessage, ServerMessage};
    use gloo::console::log;
    use yew::Callback;

    let (write, read) = socket("ws://127.0.0.1:4567");
    write.request(ClientMessage::Login {
        name: "nano",
        pass: "nano",
    });

    let state = Rc::new(RefCell::new(State::default()));

    let view = {
        let root = gloo::utils::document()
            .get_element_by_id("root")
            .expect_throw("root");

        let app = yew::start_app_with_props_in_element::<App>(
            root,
            Props {
                data: Data {
                    state: Rc::clone(&state),
                    current_channel: 0,
                    me: 0,
                },
                onaction: Callback::from(|_| todo!()),
            },
        );

        View { app }
    };

    read.register(move |message| match message {
        ServerMessage::Closed => log!("closed"),
        ServerMessage::LoggedIn(logged) => match logged {
            Ok(id) => log!("logged", id),
            Err(err) => log!("error", err.to_string()),
        },
        ServerMessage::User(_) => log!("user"),
        ServerMessage::Channel(chan) => {
            state
                .borrow_mut()
                .push_channel(Channel::new(&chan.name, chan.icon.as_deref()));

            view.update();
        }
        ServerMessage::Said { from, chan, text } => {
            state.borrow_mut().push_message(
                chan,
                Message {
                    from,
                    text: text.into(),
                },
            );

            view.update();
        }
    });

    Ok(())
}
