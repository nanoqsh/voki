mod post;
mod socket;
mod state;
mod view;

use self::{
    socket::socket,
    state::{Channel, Message, State, User},
    view::{Action, App, Data, Event, Props},
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
    use base::api::{ClientMessage, ServerMessage};
    use gloo::console::log;
    use yew::Callback;

    let (write, read) = socket("ws://0.0.0.0:4567");

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
                },
                onaction: Callback::from({
                    let write = write.clone();
                    move |action| match action {
                        Action::Send { chan, text } => {
                            write.request(ClientMessage::Say { chan, text: &text })
                        }
                    }
                }),
                onlogin: Callback::from(move |(name, pass): (String, String)| {
                    if !name.is_empty() && !pass.is_empty() {
                        write.request(ClientMessage::Login {
                            name: &name,
                            pass: &pass,
                        })
                    }
                }),
            },
        );

        View { app }
    };

    read.register(move |message| match message {
        ServerMessage::Closed => log!("closed"),
        ServerMessage::LoggedIn(logged) => match logged {
            Ok(id) => {
                state.borrow_mut().set_login(id);
                view.update();
            }
            Err(err) => {
                log!("error", err.to_string());
                state.borrow_mut().retry = true;
                view.update();
            }
        },
        ServerMessage::User(user) => {
            state.borrow_mut().push_user(
                user.id,
                User {
                    name: user.name.into(),
                    avatar: user.avatar.map(Into::into),
                },
            );

            view.update();
        }
        ServerMessage::Channel(chan) => {
            {
                let mut state = state.borrow_mut();
                state.push_channel(chan.id, Channel::new(&chan.name, chan.icon.as_deref()));
                for message in chan.history {
                    state.push_message(
                        message.chan,
                        Message {
                            from: message.from,
                            text: message.text.into(),
                        },
                    );
                }
            }

            view.update();
        }
        ServerMessage::Message(message) => {
            state.borrow_mut().push_message(
                message.chan,
                Message {
                    from: message.from,
                    text: message.text.into(),
                },
            );

            view.update();
        }
    });

    Ok(())
}
