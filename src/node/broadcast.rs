use std::{
    collections::{HashMap, HashSet},
    io::Write,
    time::{Duration, Instant},
};

use anyhow::Context;
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
    Gossip {
        messages: Vec<usize>,
    },
    GossipOk,
}

pub struct BroadcastNode {
    node_id: String,
    node_ids: Vec<String>,
    id: usize,

    vals: Vec<usize>,

    neighbors: Vec<String>,

    known: HashMap<String, HashSet<usize>>,

    last_gossip: Instant,
}

impl BroadcastNode {
    fn has_val(&self, message: usize) -> bool {
        self.vals.iter().find(|&v| *v == message).is_some()
    }

    fn apply_gossip(&mut self, write: &mut impl Write) -> anyhow::Result<()> {
        for peer in self.neighbors.iter() {
            let messages = self
                .vals
                .iter()
                .filter(|&v| {
                    if let Some(s) = self.known.get(peer) {
                        return !s.contains(v);
                    }
                    true
                })
                .map(|v| *v)
                .collect::<Vec<_>>();
            if messages.is_empty() {
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
                    payload: BroadcastPayload::Gossip { messages },
                },
            };
            gossip
                .write_and_flush(write)
                .context("Failed to write gossip message")?;
        }
        Ok(())
    }
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
            known: HashMap::new(),
            last_gossip: Instant::now(),
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

                // Update known table
                if reply.dst.starts_with('n') {
                    self.known
                        .entry(reply.dst.clone())
                        .or_default()
                        .insert(message);
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

            BroadcastPayload::Gossip { ref messages } => {
                messages.iter().for_each(|message| {
                    if !self.has_val(*message) {
                        self.vals.push(*message);
                    }
                    self.known
                        .entry(reply.dst.clone())
                        .or_default()
                        .insert(*message);
                });
                reply.body.payload = BroadcastPayload::GossipOk;
            }

            BroadcastPayload::BroadcastOk
            | BroadcastPayload::ReadOk { .. }
            | BroadcastPayload::TopologyOk
            | BroadcastPayload::GossipOk => {
                // No need to process
                return Ok(());
            }
        }
        reply.write_and_flush(write)?;

        if self.last_gossip.elapsed() > Duration::from_secs(2) {
            self.last_gossip = Instant::now();
            return self.apply_gossip(write);
        }

        Ok(())
    }

    fn id(&mut self) -> &mut usize {
        &mut self.id
    }

    fn on_timeout(&mut self, write: &mut impl Write) -> anyhow::Result<()> {
        self.apply_gossip(write)
    }
}
