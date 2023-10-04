use std::io::{StdoutLock, Write};

use anyhow::{bail, Context, Ok};
use serde::{Deserialize, Serialize};
use vortex::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

struct EchoNode {
    id: usize,
}

impl Node<Payload> for EchoNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
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
                output
                    .write_all(b"\n")
                    .context("Unable to write new line")?;
                self.id += 1;
            }
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
                output
                    .write_all(b"\n")
                    .context("Unable to write new line")?;
                self.id += 1;
            }
            Payload::EchoOk { .. } => {}
            Payload::InitOk => bail!("Should not happen"),
        }
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    main_loop(EchoNode { id: 0 })
}
