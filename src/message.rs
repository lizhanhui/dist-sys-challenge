use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,

    #[serde(rename = "dest")]
    pub dst: String,

    pub body: Body<Payload>,
}

impl<Payload> Message<Payload>
where
    Payload: Serialize,
{
    pub fn into_reply(self, id: Option<&mut usize>) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
            body: Body {
                id: id.map(|id| {
                    let mid = *id;
                    *id += 1;
                    mid
                }),
                in_reply_to: self.body.id,
                payload: self.body.payload,
            },
        }
    }

    pub fn write_and_flush(&self, write: &mut impl Write) -> anyhow::Result<()> {
        serde_json::to_writer(&mut *write, self).context("serialize message")?;
        write.write_all(b"\r\n").context("write trailing newline")?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<usize>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<usize>,

    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum InitPayload {
    Init(Init),
    InitOk,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum EchoPayload {
    Echo(Echo),
    EchoOk(Echo),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Echo {
    echo: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_echo() {
        let json = r#"{"src":"src","dest":"dst","body":{"msg_id":1,"type":"echo","echo":"echo"}}"#;

        let echo = Message::<EchoPayload> {
            src: "src".to_string(),
            dst: "dst".to_string(),
            body: Body::<EchoPayload> {
                id: Some(1),
                in_reply_to: None,
                payload: EchoPayload::Echo(Echo {
                    echo: "echo".to_string(),
                }),
            },
        };

        let echo_json = serde_json::to_string(&echo).unwrap();

        assert_eq!(json, echo_json);
    }

    #[test]
    fn test_serialize_echo_ok() {
        let json = r#"{"src":"src","dest":"dst","body":{"msg_id":1,"in_reply_to":2,"type":"echo_ok","echo":"echo"}}"#;
        let echo_ok = Message::<EchoPayload> {
            src: "src".to_string(),
            dst: "dst".to_string(),
            body: Body::<EchoPayload> {
                id: Some(1),
                in_reply_to: Some(2),
                payload: EchoPayload::EchoOk(Echo {
                    echo: "echo".to_string(),
                }),
            },
        };

        let echo_ok_json = serde_json::to_string(&echo_ok).unwrap();
        assert_eq!(json, echo_ok_json);
    }
}
