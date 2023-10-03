use std::io::{StdoutLock, Write};

use anyhow::{Context, bail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Body {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk { echo: String },
    Init { node_id: String, node_ids: Vec<String> },
    InitOk,
}

struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply).context("Unable to write to output")?;
                output.write_all(b"\n").context("Unable to write new line")?;
                self.id += 1;
            },
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply).context("Unable to write to output")?;
                output.write_all(b"\n").context("Unable to write new line")?;
                self.id += 1;   
            },
            Payload::EchoOk { .. } => {
            },
            Payload::InitOk => bail!("Should not happen"),
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();
    let mut node = EchoNode {
        id: 0
    };
    for input in inputs {
        let input = input.context("Issue in maelstrom input")?;
        node.step(input, &mut stdout ).context("step method failed")?;
    }
    Ok(())
}
