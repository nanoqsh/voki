use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: String,
}

#[function_component(Raw)]
pub fn raw(props: &Props) -> Html {
    let element = gloo::utils::document()
        .create_element("empty")
        .unwrap_throw();

    element.set_inner_html(&props.content);
    let children = element.children();

    if children.length() != 1 {
        panic!("wrong content")
    }

    let child = children.item(0).unwrap_throw();
    Html::VRef(child.into())
}
