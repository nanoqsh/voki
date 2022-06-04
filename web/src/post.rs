use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct Inner<T> {
    queue: RefCell<VecDeque<T>>,
}

#[derive(Clone)]
pub struct Sender<T> {
    inner: Rc<Inner<T>>,
}

impl<T> Sender<T> {
    pub fn push(&self, value: T) {
        self.inner.queue.borrow_mut().push_front(value);
    }
}

pub struct Receiver<T> {
    inner: Rc<Inner<T>>,
}

impl<T> Receiver<T> {
    #[must_use]
    pub fn take(&self) -> Option<T> {
        self.inner.queue.borrow_mut().pop_front()
    }

    #[must_use]
    pub fn iter(&self) -> Iter<T> {
        Iter { receiver: self }
    }
}

impl<'a, T> IntoIterator for &'a Receiver<T> {
    type Item = T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    receiver: &'a Receiver<T>,
}

impl<T> Iterator for Iter<'_, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.take()
    }
}

#[must_use]
pub fn post<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(Inner {
        queue: RefCell::new(VecDeque::default()),
    });

    (
        Sender {
            inner: Rc::clone(&inner),
        },
        Receiver { inner },
    )
}
