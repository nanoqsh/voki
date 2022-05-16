use super::raw::Raw;
use yew::prelude::*;

macro_rules! src {
    ( $path:literal ) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path))
    };
}

pub(super) use src;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub content: &'static str,
}

#[function_component(Svg)]
pub fn svg(props: &Props) -> Html {
    html! {
        <Raw content={ props.content } />
    }
}
