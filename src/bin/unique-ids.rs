use std::io::Write;

use challenge::{
    message::{Init, Message},
    node::main_loop,
    Node,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum UniqueIdPayload {
    Generate,
    GenerateOk { id: String },
}

pub struct UniqueIdNode {
    node_id: String,
    id: usize,
}

impl Node<UniqueIdPayload> for UniqueIdNode {
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
        message: Message<UniqueIdPayload>,
        write: &mut impl Write,
    ) -> anyhow::Result<()> {
        let mut reply = message.into_reply(Some(&mut self.id));
        reply.body.payload = UniqueIdPayload::GenerateOk {
            id: format!("{}-{}", self.node_id, self.id),
        };
        reply.write_and_flush(write)?;
        Ok(())
    }

    fn id(&mut self) -> &mut usize {
        &mut self.id
    }
}

fn main() -> anyhow::Result<()> {
    main_loop::<UniqueIdPayload, UniqueIdNode>()
}
