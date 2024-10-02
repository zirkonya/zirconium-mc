use serde::{Deserialize, Serialize};
use zr_core::particle::Particle;

#[derive(Clone, Serialize, Deserialize)]
pub struct Options {
    #[serde(rename = "type")]
    type_field: String,
    value: Option<Particle>,
}
