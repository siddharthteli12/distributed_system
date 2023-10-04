use std::io::{StdoutLock, Write};

use anyhow::{bail, Context, Ok};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use vortex::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Generate,
    GenerateOk {
        id: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

struct UniqueNode {
    id: usize,
}

impl Node<Payload> for UniqueNode {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Generate => {
                let reply = Message {
                    src: input.dest,
                    dest: input.src,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: input.body.msg_id,
                        payload: Payload::GenerateOk { id: random_str() },
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
            Payload::GenerateOk { .. } => bail!("Should not happen"),
            Payload::InitOk => bail!("Should not happen"),
        }
        Ok(())
    }
}

fn random_str() -> String {
    let random_str: String = thread_rng()
        .sample_iter(Alphanumeric)
        .take(thread_rng().gen_range(12..70))
        .map(char::from)
        .collect();
    random_str
}

fn main() -> anyhow::Result<()> {
    //main_loop(UniqueNode { id: 0 })
    println!("{:?}", random_str());
    Ok(())
}
