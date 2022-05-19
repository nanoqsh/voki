use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub content: String,
}

#[function_component(Raw)]
pub fn raw(props: &Props) -> Html {
    let element = gloo::utils::document()
        .create_element("empty")
        .expect_throw("create element");

    element.set_inner_html(&props.content);
    let children = element.children();

    if children.length() != 1 {
        panic!("wrong content")
    }

    let child = children.item(0).expect_throw("item");
    Html::VRef(child.into())
}
