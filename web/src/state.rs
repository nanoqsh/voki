use im::{HashMap, Vector};
use std::{fmt, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Message {
    pub from: u32,
    pub text: Rc<str>,
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
            .map(|message| message.text.as_ref())
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

            if lhs.is_inline() {
                lhs == rhs
            } else {
                lhs.ptr_eq(rhs)
            }
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
    channels: Vector<Channel>,
    users: HashMap<u32, User>,
}

impl State {
    pub fn channels(&self) -> impl Iterator<Item = &Channel> {
        self.channels.iter()
    }

    pub fn messages(&self, channel: u32) -> Vector<(u32, Vector<Rc<str>>)> {
        use itertools::Itertools;

        self.channels
            .get(channel as usize)
            .map(|channel| {
                channel
                    .messages
                    .iter()
                    .group_by(|message| message.from)
                    .into_iter()
                    .map(|(from, messages)| {
                        (
                            from,
                            messages.map(|message| &message.text).cloned().collect(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn user(&self, user: u32) -> Option<&User> {
        self.users.get(&user)
    }

    pub fn push_channel(&mut self, chan: Channel) {
        self.channels.push_back(chan);
    }

    pub fn push_message(&mut self, chan: u32, message: Message) {
        if let Some(chan) = self.channels.get_mut(chan as usize) {
            chan.messages.push_back(message);
        }
    }

    pub fn push_user(&mut self, id: u32, user: User) {
        self.users.insert(id, user);
    }
}
