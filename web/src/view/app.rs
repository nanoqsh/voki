use super::chat::Chat;
use yew::prelude::*;

#[function_component(Channels)]
fn channels() -> Html {
    html! {
        <div class="channels">
            <p>{ "todo!()" }</p>
        </div>
    }
}

pub enum Message {
    Received { from: String, text: String },
}

pub enum Action {
    Send(String),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub onaction: Callback<Action>,
}

pub struct App;

impl Component for App {
    type Message = Message;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Message::Received { .. } => {}
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsend = Callback::from({
            let onaction = ctx.props().onaction.clone();
            move |text| onaction.emit(Action::Send(text))
        });

        html! {
            <div class="app">
                <Channels />
                <Chat { onsend } />
            </div>
        }
    }
}
