use crate::logic::{Gesture, RoundResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct InitData {
    pub use_combo_scoring: bool,
}

#[derive(Serialize, Clone)]
pub struct Request {
    pub last_gestures: [Gesture; 3],
    pub last_result: [RoundResult; 3],
    pub total_scores: [i32; 3],
}

/// For our RPS, Display(to player) is the same as Request(to agent)
#[derive(Serialize, Clone)]
pub struct Display {
    pub last_gestures: [Gesture; 3],
    pub last_result: [RoundResult; 3],
    pub total_scores: [i32; 3],
}

#[derive(Deserialize)]
pub struct Response {
    pub gesture: Gesture,
}
