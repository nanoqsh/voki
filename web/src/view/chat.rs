use super::svg::{src, Svg};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MessageProps {
    avatar: Option<String>,
    name: String,
    rows: Vec<String>,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    html! {
        <div class="message">
            {
                match &props.avatar {
                    Some(image) => html! {
                        <img class="avatar" src={ image.clone() } />
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

#[derive(Properties, PartialEq)]
pub struct InputProps {
    onsend: Callback<String>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    use web_sys::{Element, HtmlTextAreaElement, InputEvent, KeyboardEvent};

    let node = NodeRef::default();
    let send = {
        let node = node.clone();
        let onsend = props.onsend.clone();
        move || {
            let text: HtmlTextAreaElement = node.cast().unwrap_throw();
            onsend.emit(text.value());
            text.set_value("");
            text.set_attribute("style", "").unwrap_throw();
        }
    };

    let oninput = Callback::from(|ev: InputEvent| {
        let element: Element = ev.target_dyn_into().unwrap_throw();
        let height = element.scroll_height();
        let height = format!("height: {}px", height.min(200));
        element.set_attribute("style", &height).unwrap_throw();
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
                <Svg content={ src!("/icons/send.svg") } />
            </div>
        </div>
    }
}

pub enum Event {
    Send { from: String, text: String },
}

struct MessageData {
    avatar: Option<String>,
    name: String,
    rows: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsend: Callback<String>,
}

pub struct Chat {
    me: String,
    messages: Vec<MessageData>,
    scroll_to_end: bool,
}

impl Chat {
    fn scroll_to_end(&mut self) {
        if self.scroll_to_end {
            let height = gloo::utils::document()
                .body()
                .unwrap_throw()
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
            me: "Claire".into(),
            messages: vec![
                MessageData {
                    avatar: Some("./images/1.jpg".into()),
                    name: "Claire".into(),
                    rows: vec!["hi!".into(), "sup?".into(), "lol".into(), "kek".into()],
                },
                MessageData {
                    avatar: None,
                    name: "Claire".into(),
                    rows: vec!["hi!".into(), "suck".into()],
                },
                MessageData {
                    avatar: Some("./images/1.jpg".into()),
                    name: "Claire".into(),
                    rows: vec!["I ❤️ you".into()],
                },
                MessageData {
                    avatar: Some("./images/1.jpg".into()),
                    name: "Claire".into(),
                    rows: vec!["I ❤️ you".into()],
                },
                MessageData {
                    avatar: Some("./images/1.jpg".into()),
                    name: "Claire_".into(),
                    rows: vec![
                        "0".into(),
                        "1".into(),
                        "2".into(),
                        "3".into(),
                        "4".into(),
                        "5".into(),
                        "6".into(),
                    ],
                },
            ],
            scroll_to_end: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::Send { from, text } if !text.trim().is_empty() => {
                match self.messages.last_mut() {
                    Some(last) if last.name == from => last.rows.push(text.clone()),
                    _ => self.messages.push(MessageData {
                        avatar: Some("./images/1.jpg".into()),
                        name: self.me.clone(),
                        rows: vec![text.clone()],
                    }),
                }

                ctx.props().onsend.emit(text);
                self.scroll_to_end = true;
            }
            _ => {}
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsend = ctx.link().callback({
            let me = self.me.clone();
            move |text| Event::Send {
                from: me.clone(),
                text,
            }
        });

        html! {
            <div class="chat">
                <div class="messages">
                    {
                        for self.messages.iter().map(|MessageData { avatar, name, rows }| html! {
                            <Message
                                avatar={ avatar.clone() }
                                name={ name.clone() }
                                rows={ rows.clone() }
                            />
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
