use serde::{Deserialize, Serialize};
use zr_nbt::either::Either;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimension {
    pub fixed_time: Option<i64>,
    pub has_skylight: i8,
    pub has_ceiling: i8,
    pub ultrawarm: i8,
    pub natural: i8,
    pub coordinate_scale: f64,
    pub bed_works: i8,
    pub respawn_anchor_works: i8,
    pub min_y: i32,
    pub height: i32,
    pub logical_height: i32,
    pub infiniburn: String,
    pub effects: String,
    pub ambient_light: f32,
    pub piglin_safe: i8,
    pub has_raids: i8,
    pub monster_spawn_light_level: Either<MonsterSpawnLightLevel, i32>,
    pub monster_spawn_block_light_limit: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MonsterSpawnLightLevel {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Value {
    pub max_inclusive: i32,
    pub min_inclusive: i32,
}
