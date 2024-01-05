use std::{
    io::{BufRead, Write},
    sync::mpsc::RecvTimeoutError,
};

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

    fn on_timeout(&mut self, write: &mut impl Write) -> anyhow::Result<()>;

    fn id(&mut self) -> &mut usize;
}

pub enum Event<Payload> {
    Message(Message<Payload>),
    Eof,
}

pub fn main_loop<P, N>() -> anyhow::Result<()>
where
    N: Node<P>,
    P: DeserializeOwned + Send + 'static,
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

    let (tx, rx) = std::sync::mpsc::channel();

    if let InitPayload::Init(init) = &init_message.body.payload {
        let mut node = N::from_init(init)?;
        let mut init_ok_message = init_message.into_reply(Some(&mut node.id()));
        init_ok_message.body.payload = InitPayload::InitOk;
        init_ok_message.write_and_flush(&mut stdout)?;

        drop(lines);

        let handle = std::thread::spawn(move || {
            let stdin = std::io::stdin().lock();
            let lines = stdin.lines();
            for line in lines {
                let message = serde_json::from_str::<Message<P>>(
                    &line
                        .context("Failed to read echo message from STDIN")
                        .unwrap(),
                )
                .context("Failed to parse echo message")
                .unwrap();
                tx.send(Event::Message(message)).unwrap();
            }
            tx.send(Event::Eof).expect("Failed to send EOF");
        });

        loop {
            match rx.recv_timeout(std::time::Duration::from_millis(100)) {
                Ok(Event::Message::<P>(m)) => {
                    node.process(m, &mut stdout)?;
                }
                Ok(Event::Eof) => {
                    node.on_timeout(&mut stdout)?;
                }
                Err(RecvTimeoutError::Timeout) => {
                    node.on_timeout(&mut stdout)?;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    break;
                }
            }
        }

        handle.join().unwrap();
        Ok(())
    } else {
        panic!("Bad first message");
    }
}
