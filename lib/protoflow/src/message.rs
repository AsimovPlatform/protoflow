// This is free and unencumbered software released into the public domain.

use crate::{prelude::Box, Message};

pub trait IntoMessage {
    fn into_message(self) -> Box<dyn Message>;
}

impl IntoMessage for Box<dyn Message> {
    fn into_message(self) -> Box<dyn Message> {
        self
    }
}

impl<T: Message + Clone + 'static> IntoMessage for Box<T> {
    fn into_message(self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}

impl<T: Message + Clone + 'static> IntoMessage for &T {
    fn into_message(self) -> Box<dyn Message> {
        Box::new(self.clone())
    }
}
