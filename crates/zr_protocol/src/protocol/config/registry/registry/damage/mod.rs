use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Damage {
    pub scaling: String,
    pub exhaustion: f64,
    pub message_id: String,
    pub effect: Option<String>,
    pub death_message_type: Option<String>,
}