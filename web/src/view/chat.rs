use crate::{
    state::MessageContent,
    view::{
        svg::{src, Svg},
        Data,
    },
};
use im::Vector;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct MessageProps {
    avatar: Option<Rc<str>>,
    name: Rc<str>,
    rows: Vector<MessageContent>,
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
                        for props.rows.iter().map(|content| match content {
                            MessageContent::Text(text) => html! {
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
                            },
                            MessageContent::File(file) =>{
                                let mut src = String::from("./images/");
                                src.push_str(file);
                                html! {
                                    <img { src } />
                                }
                            },
                        })
                    }
                </div>
            </div>
        </div>
    }
}

pub enum SendEvent {
    Text(Rc<str>),
    File(String, Vec<u8>),
}

#[derive(PartialEq, Properties)]
pub struct InputProps {
    onsend: Callback<SendEvent>,
}

#[function_component(Input)]
pub fn input(props: &InputProps) -> Html {
    let node_send = NodeRef::default();
    let send = {
        let node = node_send.clone();
        let onsend = props.onsend.clone();
        move || {
            let text: web_sys::HtmlTextAreaElement = node.cast().expect_throw("cast");
            onsend.emit(SendEvent::Text(text.value().into()));
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

    let onclick_send = Callback::from(move |_: MouseEvent| send());

    let node_attach = NodeRef::default();
    let onclick_attach = Callback::from({
        let node = node_attach.clone();
        move |_: MouseEvent| {
            let list: web_sys::HtmlElement = node.cast().expect_throw("cast");
            list.click();
        }
    });

    let onchange = Callback::from({
        let node = node_attach.clone();
        let onsend = props.onsend.clone();
        move |_: web_sys::Event| {
            let list: web_sys::HtmlInputElement = node.cast().expect_throw("cast");
            let file_list: gloo::file::FileList = list.files().expect_throw("files").into();
            let files: &[gloo::file::File] = &file_list;
            for file in files {
                let file: gloo::file::File = file.clone();
                let onsend = onsend.clone();
                wasm_futures::spawn_local(async move {
                    let file_name = file.name();
                    if let Some(ext) = file_name.split('.').last() {
                        match gloo::file::futures::read_as_bytes(&file).await {
                            Ok(bytes) => onsend.emit(SendEvent::File(ext.to_string(), bytes)),
                            Err(err) => gloo::console::log!("file read error:", err.to_string()),
                        }
                    }
                });
            }
        }
    });

    html! {
        <div class="input">
            <input
                ref={ node_attach }
                { onchange }
                style="display: none;"
                type="file"
                accept="image/jpg, image/jpeg, image/png"
            />
            <textarea ref={ node_send } { oninput } { onkeypress }></textarea>
            <div class="button" onclick={ onclick_send }>
                <Svg content={ src!("/icons/send.svg") } />
            </div>
            <div class="button" onclick={ onclick_attach }>
                <Svg content={ src!("/icons/attach.svg") } />
            </div>
        </div>
    }
}

pub enum Event {
    Send {
        channel: u32,
        text: Rc<str>,
    },
    File {
        channel: u32,
        ext: String,
        bytes: Vec<u8>,
    },
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onsend: Callback<(u32, Rc<str>)>,
    pub onfile: Callback<(u32, String, Vec<u8>)>,
}

pub struct Chat;

impl Chat {
    fn scroll_to_end(&mut self) {
        let height = gloo::utils::document()
            .body()
            .expect_throw("body")
            .scroll_height();

        gloo::utils::window().scroll_by_with_x_and_y(0., height as f64);
    }
}

impl Component for Chat {
    type Message = Event;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Event::Send { channel, text } if !text.trim().is_empty() => {
                ctx.props().onsend.emit((channel, text));
            }
            Event::File {
                channel,
                ext,
                bytes,
            } => ctx.props().onfile.emit((channel, ext, bytes)),
            _ => {}
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (data, _): (Data, _) = ctx.link().context(Callback::noop()).expect("context");

        let channel = data.current_channel;
        let onsend = ctx.link().callback(move |ev: SendEvent| match ev {
            SendEvent::Text(text) => Event::Send { channel, text },
            SendEvent::File(ext, bytes) => Event::File {
                channel,
                ext,
                bytes,
            },
        });

        let state = data.state.borrow();
        html! {
            <div class="chat">
                <div class="messages">
                    {
                        for state.messages(data.current_channel).into_iter().map(|(from, messages)| {
                            let user = state.user(from).cloned().unwrap_or_default();

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
