use crate::{
    state::State,
    view::{channels::Channels, chat::Chat},
};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Data {
    pub state: State,
    pub current_channel: u32,
    pub me: u32,
}

pub enum Event {
    Received { from: String, text: String },
}

pub enum Action {
    Send(String),
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub data: Data,
    pub onaction: Callback<Action>,
}

pub struct App;

impl Component for App {
    type Message = Event;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Event::Received { .. } => {}
        }

        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = ctx.props().data.clone();

        let onsend = Callback::from({
            let onaction = ctx.props().onaction.clone();
            move |text| onaction.emit(Action::Send(text))
        });

        html! {
            <div class="app">
                <ContextProvider<Data> { context }>
                    <Channels />
                    <Chat { onsend } />
                </ContextProvider<Data>>
            </div>
        }
    }
}
