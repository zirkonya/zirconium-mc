use serde::{Deserialize, Serialize};
use zr_nbt::either::Either;

use crate::protocol::config::registry::Description;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorTrimMaterial {
    pub asset_name: String,
    pub ingredient: String,
    pub item_model_index: f32,
    pub override_armor_materials: Option<OverrideArmorMaterials>,
    pub description: Either<String, Description>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverrideArmorMaterials {
    pub leather: Option<String>,
    pub chainmail: Option<String>,
    pub iron: Option<String>,
    pub gold: Option<String>,
    pub diamond: Option<String>,
    pub turtle: Option<String>,
    pub netherite: Option<String>,
}
