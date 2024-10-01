use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zr_binary_macros::Binary;
use zr_nbt::either::Either;

#[derive(Binary, Deserialize, Serialize)]
pub struct ArmorTrimMaterial {
    pub asset_name: String,
    pub ingredient: String,
    pub item_model_index: f32,
    pub override_armor_materials: Option<HashMap<String, String>>,
    pub description: Either<String, HashMap<String, String>>,
}
