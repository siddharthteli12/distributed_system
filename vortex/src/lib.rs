use std::io::StdoutLock;

use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message<Payload> {
    pub src: String,
    pub dest: String,
    pub body: Body<Payload>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Body<Payload> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

pub trait Node<Payload> {
    fn step(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message<Payload>>();
    for input in inputs {
        let input = input.context("Issue in maelstrom input")?;
        state
            .step(input, &mut stdout)
            .context("step method failed")?;
    }
    Ok(())
}
