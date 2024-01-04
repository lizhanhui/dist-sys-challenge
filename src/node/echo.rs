use super::Node;
use crate::message::Init;

pub struct EchoNode {
    // Node ID
    pub node_id: String,
    pub id: usize,
}

impl Node for EchoNode {
    fn from_init(init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            node_id: init.node_id.clone(),
            id: 0,
        })
    }
}
