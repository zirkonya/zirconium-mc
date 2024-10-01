use serde::{Deserialize, Serialize};
use zr_binary_macros::Binary;

pub mod armor_trim;
pub mod banner_pattern;
pub mod biome;
pub mod chat_type;
pub mod damage_type;
pub mod dimension_type;
pub mod painting_variant;
pub mod wolf_variant;

#[derive(Clone, Binary, Serialize, Deserialize)]
pub struct Registry {}
