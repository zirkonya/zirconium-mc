use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AmbientSound {
    pub sound_id: String,
    pub range: Option<f32>,
}

