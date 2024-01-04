use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};

use crate::{
    message::{Init, Message},
    Node,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BroadcastPayload {
    Broadcast {
        message: usize,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<usize>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

pub struct BroadcastNode {
    node_id: String,
    node_ids: Vec<String>,
    id: usize,

    vals: Vec<usize>,
}

impl Node<BroadcastPayload> for BroadcastNode {
    fn from_init(init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            node_id: init.node_id.clone(),
            node_ids: init.node_ids.clone(),
            id: 0,
            vals: Vec::new(),
        })
    }

    fn process(
        &mut self,
        message: Message<BroadcastPayload>,
        write: &mut impl Write,
    ) -> anyhow::Result<()> {
        let mut reply = message.into_reply(Some(&mut self.id));

        match reply.body.payload {
            BroadcastPayload::Broadcast { message } => {
                if self.vals.iter().find(|&v| *v == message).is_none() {
                    self.vals.push(message);
                }
                reply.body.payload = BroadcastPayload::BroadcastOk;
            }
            BroadcastPayload::Read => {
                reply.body.payload = BroadcastPayload::ReadOk {
                    messages: self.vals.clone(),
                };
            }
            BroadcastPayload::Topology { topology: _ } => {
                reply.body.payload = BroadcastPayload::TopologyOk;
            }

            BroadcastPayload::BroadcastOk
            | BroadcastPayload::ReadOk { .. }
            | BroadcastPayload::TopologyOk => {
                // No need to process
            }
        }
        reply.write_and_flush(write)?;
        Ok(())
    }

    fn id(&mut self) -> &mut usize {
        &mut self.id
    }
}
