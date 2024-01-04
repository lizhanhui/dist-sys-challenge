use challenge::node::{
    broadcast::{BroadcastNode, BroadcastPayload},
    main_loop,
};

fn main() -> anyhow::Result<()> {
    main_loop::<BroadcastPayload, BroadcastNode>()
}
