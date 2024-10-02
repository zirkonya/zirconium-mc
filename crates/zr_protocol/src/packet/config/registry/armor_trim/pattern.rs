use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use zr_binary_macros::Binary;
use zr_nbt::either::Either;

#[derive(Clone, Serialize, Deserialize)]
pub struct ArmorTrimPattern {
    pub asset_id: String,
    pub template_item: String,
    pub description: Either<String, HashMap<String, String>>,
    pub decal: i8,
}
