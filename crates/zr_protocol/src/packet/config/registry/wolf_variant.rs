use serde::{Deserialize, Serialize};
use zr_nbt::either::Either;

#[derive(Clone, Serialize, Deserialize)]
pub struct WolfVariant {
    wild_texture: String,
    tame_texture: String,
    angry_texture: String,
    biomes: Either<String, Vec<String>>,
}
