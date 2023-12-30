use crate::message::{Body, Message, Type};

pub struct Node<'a> {
    writer: &'a mut dyn std::io::Write,

    // The node_id field indicates the ID of the node which is receiving this message: here, the node ID is "n3". Your node should remember this ID and include it as the src of any message it sends.
    id: Option<String>,

    // unique-ids
    seq: usize,
    step: usize,

    // broadcast
    messages: Vec<usize>,
}

impl<'a> Node<'a> {
    pub fn new(writer: &'a mut dyn std::io::Write) -> Self {
        Node {
            writer,
            id: None,
            seq: 0,
            step: 0,
            messages: vec![],
        }
    }

    pub fn step(&mut self, message: Message) -> anyhow::Result<()> {
        match message.body.ty {
            Type::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                let num = node_id.as_str();
                self.seq = (&num[1..]).parse::<usize>()?;

                self.id = Some(node_id);
                self.step = node_ids.len();
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
            Type::Generate { msg_id } => {
                self.seq += self.step;
                let generate_ok = Message {
                    src: message.dst,
                    dst: message.src,
                    body: Body {
                        ty: Type::GenerateOk {
                            msg_id,
                            in_reply_to: msg_id,
                            id: self.seq,
                        },
                    },
                };
                self.print(generate_ok)?;
            }
            Type::GenerateOk { .. } => todo!(),

            Type::Broadcast { msg_id, msg } => {
                self.messages.push(msg);
                let broadcast_ok = Message {
                    src: message.dst,
                    dst: message.src,
                    body: Body {
                        ty: Type::BroadcastOk {
                            msg_id,
                            in_reply_to: msg_id,
                        },
                    },
                };
                self.print(broadcast_ok)?;
            }
            Type::BroadcastOk { .. } => {}

            Type::Read { msg_id } => {
                let read_ok = Message {
                    src: message.dst,
                    dst: message.src,
                    body: Body {
                        ty: Type::ReadOk {
                            msg_id,
                            in_reply_to: msg_id,
                            messages: self.messages.clone(),
                        },
                    },
                };
                self.print(read_ok)?;
            }
            Type::ReadOk { .. } => {}
            Type::Topology { msg_id, topology } => {
                let topology_ok = Message {
                    src: message.dst,
                    dst: message.src,
                    body: Body {
                        ty: Type::TopologyOk {
                            msg_id,
                            in_reply_to: msg_id,
                        },
                    },
                };
                self.print(topology_ok)?;
            }
            Type::TopologyOk { .. } => {}
        }
        Ok(())
    }

    fn print(&mut self, message: Message) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *self.writer, &message)?;
        self.writer.write_all(b"\r\n")?;
        Ok(())
    }
}
