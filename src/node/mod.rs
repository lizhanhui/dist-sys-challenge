use crate::message::Init;

pub mod echo;

pub trait Node {
    fn from_init(init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized;
}
