use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AmbientSound {
    sound_id: String,
    range: Option<f32>,
}
