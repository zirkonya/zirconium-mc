pub mod option;
use option::Options;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Particle {
    options: Options,
    probability: f32,
}
