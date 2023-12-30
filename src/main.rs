use anyhow::Context;
use challenge::message::Message;
use serde_json::Deserializer;

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();

    let mut node = challenge::node::Node::new(&mut stdout);

    let messages = Deserializer::from_reader(stdin).into_iter::<Message>();

    for message in messages {
        let message = message.context("Failed to deserialize input to message")?;
        node.step(message)?;
    }

    Ok(())
}
