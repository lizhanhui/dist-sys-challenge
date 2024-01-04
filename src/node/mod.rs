use std::io::{BufRead, Write};

use anyhow::Context;
use serde::de::DeserializeOwned;

use crate::message::{Init, InitPayload, Message};

pub mod broadcast;
pub mod echo;
pub mod unique_ids;

pub trait Node<Payload> {
    fn from_init(init: &Init) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn process(&mut self, message: Message<Payload>, write: &mut impl Write) -> anyhow::Result<()>;

    fn id(&mut self) -> &mut usize;
}

pub fn main_loop<P, N>() -> anyhow::Result<()>
where
    N: Node<P>,
    P: DeserializeOwned,
{
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
        let mut node = N::from_init(init)?;
        let mut init_ok_message = init_message.into_reply(Some(&mut node.id()));
        init_ok_message.body.payload = InitPayload::InitOk;
        init_ok_message.write_and_flush(&mut stdout)?;

        for line in lines {
            let echo_message = serde_json::from_str::<Message<P>>(
                &line.context("Failed to read echo message from STDIN")?,
            )
            .context("Failed to parse echo message")?;
            node.process(echo_message, &mut stdout)?;
        }

        Ok(())
    } else {
        panic!("Bad first message");
    }
}
