use crate::message::{Body, Message, Type};

pub struct Node<'a> {
    writer: &'a mut dyn std::io::Write,

    // The node_id field indicates the ID of the node which is receiving this message: here, the node ID is "n3". Your node should remember this ID and include it as the src of any message it sends.
    id: Option<String>,
}

impl<'a> Node<'a> {
    pub fn new(writer: &'a mut dyn std::io::Write) -> Self {
        Node { writer, id: None }
    }

    pub fn step(&mut self, message: Message) -> anyhow::Result<()> {
        match message.body.ty {
            Type::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                let init_ok = Message {
                    src: message.dst,
                    dst: message.src,
                    body: Body {
                        ty: Type::InitOk {
                            in_reply_to: msg_id,
                        },
                    },
                };
                self.print(init_ok)?;
            }
            Type::InitOk { .. } => {}
            Type::Echo { msg_id, echo } => {
                let echo_ok = Message {
                    src: message.dst,
                    dst: message.src,
                    body: Body {
                        ty: Type::EchoOk {
                            msg_id,
                            in_reply_to: msg_id,
                            echo,
                        },
                    },
                };
                self.print(echo_ok)?;
            }
            Type::EchoOk { .. } => {}
        }
        Ok(())
    }

    fn print(&mut self, message: Message) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *self.writer, &message)?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }
}
