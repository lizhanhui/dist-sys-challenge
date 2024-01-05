use std::{collections::HashMap, io::Write};

use serde::{Deserialize, Serialize};

use crate::{
    message::{Body, Init, Message},
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

    neighbors: Vec<String>,
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
            neighbors: Vec::new(),
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
                let mut new = false;
                if self.vals.iter().find(|&v| *v == message).is_none() {
                    self.vals.push(message);
                    new = true;
                }

                if new {
                    for peer in &self.neighbors {
                        // Skip the original node
                        if reply.dst == *peer {
                            continue;
                        }
                        let mid = self.id;
                        self.id += 1;
                        let gossip = Message::<BroadcastPayload> {
                            src: self.node_id.clone(),
                            dst: peer.clone(),
                            body: Body::<BroadcastPayload> {
                                id: Some(mid),
                                in_reply_to: None,
                                payload: BroadcastPayload::Broadcast { message: message },
                            },
                        };
                        // broadcast new message to its neighbours.
                        gossip.write_and_flush(&mut *write)?;
                    }
                }

                reply.body.payload = BroadcastPayload::BroadcastOk;
            }
            BroadcastPayload::Read => {
                reply.body.payload = BroadcastPayload::ReadOk {
                    messages: self.vals.clone(),
                };
            }
            BroadcastPayload::Topology { topology } => {
                if let Some(n) = topology.get(&self.node_id) {
                    self.neighbors.append(&mut n.clone());
                }
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
