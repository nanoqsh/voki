mod view;

use self::view::{App, Message, Props};
use wasm_bindgen::prelude::*;
use yew::AppHandle;

struct State {
    app: AppHandle<App>,
}

impl State {
    fn received(&self, from: String, text: String) {
        self.app.send_message(Message::Received { from, text });
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use yew::Callback;

    let root = gloo::utils::document()
        .get_element_by_id("root")
        .unwrap_throw();

    let app = yew::start_app_with_props_in_element::<App>(
        root,
        Props {
            onaction: Callback::from(|_| todo!()),
        },
    );

    let state = State { app };
    state.received("nano".into(), "hi".into());

    Ok(())
}
