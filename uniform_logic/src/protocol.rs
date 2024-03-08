use crate::serialize_map::SerializableMap;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::collections::HashMap;

/// Possible verdict of an agent by Judger
#[derive(Deserialize, PartialEq, Eq, Debug)]
pub enum AgentVerdict {
    OK,
    RE,
    TLE,
    MLE,
    OLE,
    STLE,
    EXIT,
    UE,
    CANCEL,
    IA,
}

/// Message sent from Agent to Logic
#[derive(Deserialize)]
pub struct AgentMessage<Response> {
    pub verdict: AgentVerdict,
    pub response: Response,
}

/// Message sent from Logic to Agent at the game ending
#[derive(Serialize)]
pub struct FinishMessage {
    pub score: f32,
    pub state: String,
}

/// Init message sent from Judger to Logic
#[derive(Deserialize)]
pub struct InitInput<InitData> {
    pub initdata: InitData,
}

/// Message sent from Logic to Agent in each round
///
/// Normally, the message is a request for agents to response
pub struct RoundOutput<Request, Display> {
    pub content: SerializableMap<String, Request>, // Agent name, Request
    pub display: Display,                          // Round display for player
}

/// Message sent from Agent to Logic in each round
///
/// Normally, the message is a response to Logic's request
#[derive(Deserialize)]
pub struct RoundInput<Response> {
    pub log: SerializableMap<String, AgentMessage<Response>>, // Agent name, Response
}

/// Finish message sent from Logic to Judger
pub struct FinishOutput<Display> {
    pub content: HashMap<String, FinishMessage>, // Agent name, Finish message
    pub display: Display,                        // Final display for player
}

// Custom impl Serialize for Request to add implicit "command"
impl<Request, Display> Serialize for RoundOutput<Request, Display>
where
    Request: Serialize,
    Display: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 3)?;
        // Add a custom field "command": "request"
        state.serialize_field("command", "request")?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("display", &self.display)?;
        state.end()
    }
}

// Custom impl Serialize for Finish to add implicit "command"
impl<Display> Serialize for FinishOutput<Display>
where
    Display: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Request", 3)?;
        // Add a custom field "command": "finish"
        state.serialize_field("command", "finish")?;
        state.serialize_field("content", &self.content)?;
        state.serialize_field("display", &self.display)?;
        state.end()
    }
}
