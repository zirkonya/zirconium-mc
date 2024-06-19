use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Particle {
    pub options: Options,
    pub probability: f32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Options {
    #[serde(rename = "type")]
    pub type_field: String,
}