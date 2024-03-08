//! Library of the Uniform Protocol
//!
//! The Uniform Protocol has 4 basic structs: [`InitInput`], [`RoundOutput`], [`RoundInput`], [`FinishOutput`].
//! The suffix "Input" means the message is sent from Judger(Agent) to Logic, while "Output" means the message
//! is sent from Logic to Judger(Agent).
//!
//! Feel free to use the 4 basic structs. You also need to design your custom protocol and define your structs
//! to specialize the general types defined in these basic structs.
//! See `InitData`, `Response`, `Request`, `Display`

mod protocol;
mod serialize_map;

pub use protocol::*;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{to_string, Error};
pub use serialize_map::SerializableMap;
use std::{collections::HashMap, io::stdin};

pub fn receive_init_message<InitData>() -> Result<InitData, Error>
where
    InitData: DeserializeOwned,
{
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    serde_json::from_str::<InitInput<InitData>>(&input_str)
        .map(|init_input| init_input.initdata)
}

pub struct RoundMessageSender<Request, Display> {
    content: HashMap<String, Request>,
    display: Option<Display>,
}

impl<Request, Display> RoundMessageSender<Request, Display>
where
    Request: Serialize,
    Display: Serialize,
{
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            display: None,
        }
    }

    pub fn send_agent(mut self, name: String, request: Request) -> Self {
        self.content.insert(name, request);
        self
    }

    pub fn send_display(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }

    /// # Panics
    /// Panics if `self.display` is `None`, `send_display` must be called before.
    pub fn end(self) -> Result<(), Error> {
        let round_output = RoundOutput {
            content: self.content.into(),
            display: self.display.unwrap(),
        };
        println!("{}", to_string(&round_output)?);
        Ok(())
    }
}

pub fn recieve_round_message<Response>() -> Result<HashMap<String, AgentMessage<Response>>, Error>
where
    Response: DeserializeOwned,
{
    let mut input_str = String::new();
    stdin().read_line(&mut input_str).unwrap();
    serde_json::from_str::<RoundInput<Response>>(&input_str)
        .map(|round_input| round_input.log.0)
}

pub struct FinishMessageSender<Display> {
    content: HashMap<String, FinishMessage>,
    display: Option<Display>,
}

impl<Display> FinishMessageSender<Display>
where
    Display: Serialize,
{
    pub fn new() -> Self {
        Self {
            content: HashMap::new(),
            display: None,
        }
    }

    pub fn send_agent(mut self, name: String, score: f32, state: String) -> Self {
        self.content.insert(name, FinishMessage { score, state });
        self
    }

    pub fn send_display(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }

    /// # Panics
    /// Panics if `self.display` is `None`, `send_display` must be called before.
    pub fn end(self) -> Result<(), Error> {
        let finish_output = FinishOutput {
            content: self.content.into(),
            display: self.display.unwrap(),
        };
        println!("{}", to_string(&finish_output)?);
        Ok(())
    }
}
