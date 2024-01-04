use challenge::{
    message::EchoPayload,
    node::{echo::EchoNode, main_loop},
};

fn main() -> anyhow::Result<()> {
    main_loop::<EchoPayload, EchoNode>()
}
