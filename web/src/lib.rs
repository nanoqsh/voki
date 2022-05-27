mod state;
mod view;

use self::{
    state::{Channel, State},
    view::{App, Data, Event, Props},
};
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
    use yew::Callback;

    let mut state = State::default();
    state.push_channel(Channel::new("Chatting", None));
    state.push_channel(Channel::new("Coding", None));

    let root = gloo::utils::document()
        .get_element_by_id("root")
        .expect_throw("root");

    let app = yew::start_app_with_props_in_element::<App>(
        root,
        Props {
            data: Data {
                state: state.clone(),
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
