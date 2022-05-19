mod view;

use self::view::{App, Message, Props};
use wasm_bindgen::prelude::*;
use yew::AppHandle;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct View {
    app: AppHandle<App>,
}

impl View {
    fn received(&self, from: String, text: String) {
        self.app.send_message(Message::Received { from, text });
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    use yew::Callback;

    let root = gloo::utils::document()
        .get_element_by_id("root")
        .expect_throw("root");

    let app = yew::start_app_with_props_in_element::<App>(
        root,
        Props {
            onaction: Callback::from(|_| todo!()),
        },
    );

    let view = View { app };
    view.received("nano".into(), "hi".into());

    Ok(())
}
