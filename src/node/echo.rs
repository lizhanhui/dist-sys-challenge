use std::io::Write;

use serde::Serialize;

use super::Node;
use crate::message::{EchoPayload, Init, Message};

pub struct EchoNode {
    // Node ID
    pub node_id: String,

    // Sequential message-id
    pub id: usize,
}

impl Node<EchoPayload> for EchoNode {
    fn from_init(init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            node_id: init.node_id.clone(),
            id: 0,
        })
    }

    fn process(
        &mut self,
        message: Message<EchoPayload>,
        write: &mut impl Write,
    ) -> anyhow::Result<()> {
        let mut echo_reply = message.into_reply(Some(&mut self.id));
        if let EchoPayload::Echo(echo) = echo_reply.body.payload {
            echo_reply.body.payload = EchoPayload::EchoOk(echo);
        }
        echo_reply.write_and_flush(&mut *write)?;
        Ok(())
    }
}
