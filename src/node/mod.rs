use std::io::Write;

use crate::message::{Init, Message};

pub mod echo;

pub trait Node<Payload> {
    fn from_init(init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn process(&mut self, message: Message<Payload>, write: &mut impl Write) -> anyhow::Result<()>;
}
