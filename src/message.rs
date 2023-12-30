use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: String,

    #[serde(rename = "dest")]
    pub dst: String,

    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "type", flatten)]
    pub ty: Type,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Type {
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk {
        in_reply_to: usize,
    },
    Echo {
        msg_id: usize,
        echo: String,
    },
    EchoOk {
        msg_id: usize,
        in_reply_to: usize,
        echo: String,
    },
    Generate {
        msg_id: usize,
    },
    GenerateOk {
        msg_id: usize,
        in_reply_to: usize,
        id: usize,
    },
    Broadcast {
        msg_id: usize,

        #[serde(rename = "message")]
        msg: usize,
    },
    BroadcastOk {
        msg_id: usize,
        in_reply_to: usize,
    },
    Read {
        msg_id: usize,
    },
    ReadOk {
        msg_id: usize,
        in_reply_to: usize,
        messages: Vec<usize>,
    },
    Topology {
        msg_id: usize,
        topology: HashMap<String, serde_json::Value>,
    },
    TopologyOk {
        msg_id: usize,
        in_reply_to: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_echo() {
        let json = r#"{"src":"src","dest":"dst","body":{"type":"echo","msg_id":1,"echo":"echo"}}"#;

        let echo = Message {
            src: "src".to_string(),
            dst: "dst".to_string(),
            body: Body {
                ty: Type::Echo {
                    msg_id: 1,
                    echo: "echo".to_string(),
                },
            },
        };

        let echo_json = serde_json::to_string(&echo).unwrap();

        assert_eq!(json, echo_json);
    }

    #[test]
    fn test_serialize_echo_ok() {
        let json = r#"{"src":"src","dest":"dst","body":{"type":"echo_ok","msg_id":1,"in_reply_to":2,"echo":"echo"}}"#;
        let echo_ok = Message {
            src: "src".to_string(),
            dst: "dst".to_string(),
            body: Body {
                ty: Type::EchoOk {
                    msg_id: 1,
                    in_reply_to: 2,
                    echo: "echo".to_string(),
                },
            },
        };

        let echo_ok_json = serde_json::to_string(&echo_ok).unwrap();
        assert_eq!(json, echo_ok_json);
    }
}
