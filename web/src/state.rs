use base::api::MessageType;
use im::{HashMap, OrdMap, Vector};
use std::{fmt, rc::Rc};

#[derive(Clone, PartialEq)]
pub enum MessageContent {
    Text(Rc<str>),
    File(Rc<str>),
}

impl From<MessageType> for MessageContent {
    fn from(message: MessageType) -> Self {
        match message {
            MessageType::Text(text) => MessageContent::Text(text.into()),
            MessageType::File(file) => MessageContent::File(file.into()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Message {
    pub from: u32,
    pub content: MessageContent,
}

#[derive(Clone)]
pub struct Channel {
    name: Rc<str>,
    icon: Option<Rc<str>>,
    messages: Vector<Message>,
}

impl Channel {
    pub fn new(name: &str, icon: Option<&str>) -> Self {
        Self {
            name: name.into(),
            icon: icon.map(Into::into),
            messages: Vector::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(Rc::as_ref)
    }

    pub fn last_message(&self) -> LastMessage {
        self.messages
            .last()
            .map(|message| match &message.content {
                MessageContent::Text(text) => text.as_ref(),
                MessageContent::File(_) => "..",
            })
            .unwrap_or_default()
            .into()
    }
}

pub struct LastMessage<'a>(&'a str);

impl LastMessage<'_> {
    const LEN: usize = 14;
}

impl fmt::Display for LastMessage<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.chars().count() {
            0 => Ok(()),
            Self::LEN => write!(f, "{} ..", self.0),
            _ => write!(f, "{}", self.0),
        }
    }
}

impl<'a> From<&'a str> for LastMessage<'a> {
    fn from(text: &'a str) -> Self {
        if text.trim().is_empty() {
            Self("")
        } else {
            let len = text.chars().take(Self::LEN).map(char::len_utf8).sum();
            Self(&text[..len])
        }
    }
}

impl PartialEq for Channel {
    fn eq(&self, rhs: &Self) -> bool {
        fn possibly_eq<T: Clone + PartialEq>(lhs: &Vector<T>, rhs: &Vector<T>) -> bool {
            if lhs.len() != rhs.len() {
                return false;
            }

            lhs.is_inline()
                .then(|| lhs == rhs)
                .unwrap_or_else(|| lhs.ptr_eq(rhs))
        }

        self.name == rhs.name && self.icon == rhs.icon && possibly_eq(&self.messages, &rhs.messages)
    }
}

#[derive(Clone, PartialEq)]
pub struct User {
    pub name: Rc<str>,
    pub avatar: Option<Rc<str>>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: "unknown".into(),
            avatar: None,
        }
    }
}

#[derive(Default, PartialEq)]
pub struct State {
    channels: OrdMap<u32, Channel>,
    users: HashMap<u32, User>,
    pub retry: bool,
    login: Option<u32>,
}

impl State {
    pub fn login(&self) -> Option<u32> {
        self.login
    }

    pub fn set_login(&mut self, id: u32) {
        self.login = Some(id);
    }

    pub fn channels(&self) -> impl Iterator<Item = &Channel> {
        self.channels.values()
    }

    pub fn messages(&self, chan: u32) -> Vector<(u32, Vector<MessageContent>)> {
        use itertools::Itertools;

        self.channels
            .get(&chan)
            .map(|chan| {
                chan.messages
                    .iter()
                    .group_by(|message| message.from)
                    .into_iter()
                    .map(|(from, messages)| {
                        (
                            from,
                            messages.map(|message| &message.content).cloned().collect(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn user(&self, user: u32) -> Option<&User> {
        self.users.get(&user)
    }

    pub fn push_channel(&mut self, id: u32, chan: Channel) {
        self.channels.insert(id, chan);
    }

    pub fn push_message(&mut self, chan: u32, message: Message) {
        if let Some(chan) = self.channels.get_mut(&chan) {
            chan.messages.push_back(message);
        }
    }

    pub fn push_user(&mut self, id: u32, user: User) {
        self.users.insert(id, user);
    }
}
