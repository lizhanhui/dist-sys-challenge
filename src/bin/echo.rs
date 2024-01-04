use std::io::BufRead;

use anyhow::Context;
use challenge::{
    message::{EchoPayload, InitPayload, Message},
    node::echo::EchoNode,
    Node,
};

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();

    let mut lines = stdin.lines();

    let mut stdout = std::io::stdout().lock();

    let init_message: Message<InitPayload> = serde_json::from_str(
        &lines
            .next()
            .expect("No init message received")
            .context("Failed to read init message from stdin")?,
    )?;

    if let InitPayload::Init(init) = &init_message.body.payload {
        let mut node = EchoNode::from_init(init)?;
        let mut init_ok_message = init_message.into_reply(Some(&mut node.id));
        init_ok_message.body.payload = InitPayload::InitOk;
        init_ok_message.write_and_flush(&mut stdout)?;

        for line in lines {
            let echo_message = serde_json::from_str::<Message<EchoPayload>>(
                &line.context("Failed to read echo message from STDIN")?,
            )
            .context("Failed to parse echo message")?;
            let mut echo_reply = echo_message.into_reply(Some(&mut node.id));
            if let EchoPayload::Echo(echo) = echo_reply.body.payload {
                echo_reply.body.payload = EchoPayload::EchoOk(echo);
            }
            echo_reply.write_and_flush(&mut stdout)?;
        }

        Ok(())
    } else {
        panic!("Bad first message");
    }
}
