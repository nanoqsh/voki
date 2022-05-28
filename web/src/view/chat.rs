use crate::view::{
    svg::{src, Svg},
    Data,
};
use im::Vector;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MessageProps {
    avatar: Option<Rc<str>>,
    name: Rc<str>,
    rows: Vector<Rc<str>>,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    html! {
        <div class="message">
            {
                match &props.avatar {
                    Some(image) => html! {
                        <img class="avatar" src={ image.to_string() } />
                    },
                    None => html! {
                        <div class="avatar"/>
                    },
                }
            }
            <div>
                <p class="name">{ props.name.clone() }</p>
                <div class="rows">
                    {
                        for props.rows.iter().map(|text| html! {
                            <p class="text">
                                {
                                    for text.lines().map(|line| html! {
                                        <>
                                            { line.trim() }
                                            <br />
                                        </>
                                    })
                                }
                            </p>
                        })
                    }
                </div>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct InputProps {
    onsend: Callback<Rc<str>>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let node = NodeRef::default();
    let send = {
        let node = node.clone();
        let onsend = props.onsend.clone();
        move || {
            let text: web_sys::HtmlTextAreaElement = node.cast().expect_throw("cast");
            onsend.emit(text.value().into());
            text.set_value("");
            text.set_attribute("style", "")
                .expect_throw("set attribute");
        }
    };

    let oninput = Callback::from(|ev: InputEvent| {
        let element: web_sys::Element = ev.target_dyn_into().expect_throw("target");
        let height = element.scroll_height();
        let height = format!("height: {}px", height.min(200));
        element
            .set_attribute("style", &height)
            .expect_throw("set attribute");
    });

    let onkeypress = Callback::from({
        let send = send.clone();
        move |ev: KeyboardEvent| {
            const ENTER: u32 = 13;

            if ev.key_code() == ENTER && !ev.shift_key() {
                ev.prevent_default();
                send();
            }
        }
    });

    let onclick = Callback::from(move |_: MouseEvent| send());

    html! {
        <div class="input">
            <textarea ref={ node } { oninput } { onkeypress }></textarea>
            <div class="button" onclick={ onclick.clone() }>
                <Svg content={ src!("/icons/send.svg") } />
            </div>
            <div class="button" { onclick }>
                <Svg content={ src!("/icons/attach.svg") } />
            </div>
        </div>
    }
}

pub enum Event {
    Send { channel: u32, text: Rc<str> },
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onsend: Callback<(u32, Rc<str>)>,
}

pub struct Chat {
    scroll_to_end: bool,
}

impl Chat {
    fn scroll_to_end(&mut self) {
        if self.scroll_to_end {
            let height = gloo::utils::document()
                .body()
                .expect_throw("body")
                .scroll_height();

            gloo::utils::window().scroll_by_with_x_and_y(0., height as f64);

            self.scroll_to_end = false;
        }
    }
}

impl Component for Chat {
    type Message = Event;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            scroll_to_end: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::Send { channel, text } if !text.trim().is_empty() => {
                ctx.props().onsend.emit((channel, text));
                self.scroll_to_end = true;
            }
            _ => {}
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (data, _): (Data, _) = ctx.link().context(Callback::noop()).expect("context");

        let channel = data.current_channel;
        let onsend = ctx
            .link()
            .callback(move |text| Event::Send { channel, text });

        html! {
            <div class="chat">
                <div class="messages">
                    {
                        for data.state.messages(data.current_channel).into_iter().map(|(from, messages)| {
                            let user = data.state.user(from).cloned().unwrap_or_default();

                            html! {
                                <Message
                                    avatar={ user.avatar }
                                    name={ user.name }
                                    rows={ messages.clone() }
                                />
                            }
                        })
                    }
                </div>
                <div class="pad"/>
                <Input { onsend } />
            </div>
        }
    }

    fn rendered(&mut self, _: &Context<Self>, _: bool) {
        self.scroll_to_end();
    }
}
