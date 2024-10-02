use serde::{Deserialize, Serialize};
use zr_nbt::either::Either;

#[derive(Clone, Serialize, Deserialize)]
pub struct Probability {
    min_inclusive: i32,
    max_inclusive: i32,
    #[serde(rename = "type")]
    type_field: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DimensionType {
    fixed_time: Option<i64>,
    has_skylight: i8,
    has_ceiling: i8,
    ultrawarm: i8,
    natural: i8,
    coordinate_scale: f64,
    bed_works: i8,
    respawn_anchor_works: i8,
    min_y: i32,
    height: i32,
    logical_height: i32,
    infiniburn: String,
    effects: String,
    ambient_light: f32,
    piglin_safe: i8,
    has_raids: i8,
    // TODO : monster_spawn_light_level
    monster_spawn_light_level: Either<i32, Probability>,
    monster_spawn_block_light_limit: i32,
}
