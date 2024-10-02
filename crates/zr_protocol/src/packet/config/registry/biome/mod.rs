pub mod effects;
pub mod particle;
pub mod sound;

use effects::Effects;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Biome {
    has_precipitation: i8,
    temperature: f32,
    temperature_modifier: Option<String>,
    downfall: f32,
    effects: Effects,
}
