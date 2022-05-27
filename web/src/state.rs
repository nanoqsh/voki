use im::{HashMap, Vector};

#[derive(Clone, PartialEq)]
pub struct Message {
    from: u32,
    text: Box<str>,
}

#[derive(Clone)]
pub struct Channel {
    name: Box<str>,
    icon: Option<Box<str>>,
    messages: Vector<Message>,
}

impl Channel {
    pub fn new(name: &str, icon: Option<&str>) -> Self {
        Self {
            name: name.into(),
            icon: icon.map(|icon| icon.into()),
            messages: Vector::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn icon(&self) -> Option<&str> {
        self.icon.as_ref().map(Box::as_ref)
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
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
    name: Box<str>,
    avatar: Option<Box<str>>,
}

#[derive(Clone, Default, PartialEq)]
pub struct State {
    channels: Vector<Channel>,
    users: HashMap<u32, User>,
}

impl State {
    pub fn channels(&self) -> impl Iterator<Item = &Channel> {
        self.channels.iter()
    }

    pub fn push_channel(&mut self, channel: Channel) {
        self.channels.push_front(channel);
    }

    pub fn push_message(&mut self, channel: u32, message: Message) {
        if let Some(channel) = self.channels.get_mut(channel as usize) {
            channel.messages.push_front(message);
        }
    }
}
