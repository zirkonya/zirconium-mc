use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DamageType {
    message_id: String,
    scaling: String,
    exhaustion: f32,
    effects: Option<String>,
    death_message_type: Option<String>,
}
