use crate::{
    state::State,
    view::{channels::Channels, chat::Chat},
};
use std::{cell::RefCell, rc::Rc};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Data {
    pub state: Rc<RefCell<State>>,
    pub current_channel: u32,
    pub me: u32,
}

pub enum Event {
    StateUpdated,
    ChannelSelected(u32),
}

pub enum Action {
    Send { chan: u32, text: Rc<str> },
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub data: Data,
    pub onaction: Callback<Action>,
}

pub struct App {
    data: Data,
}

impl Component for App {
    type Message = Event;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: ctx.props().data.clone(),
        }
    }

    fn update(&mut self, _: &Context<Self>, message: Self::Message) -> bool {
        match message {
            Event::StateUpdated => {}
            Event::ChannelSelected(index) => self.data.current_channel = index,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = self.data.clone();

        let onselect = ctx.link().callback(Event::ChannelSelected);

        let onsend = Callback::from({
            let onaction = ctx.props().onaction.clone();
            move |(chan, text)| onaction.emit(Action::Send { chan, text })
        });

        html! {
            <div class="app">
                <ContextProvider<Data> { context }>
                    <Channels { onselect } />
                    <Chat { onsend } />
                </ContextProvider<Data>>
            </div>
        }
    }
}
