use serde_derive::{Deserialize, Serialize};
use zr_nbt::either::Either;

use crate::protocol::config::registry::Description;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArmorTrimPattern {
    pub asset_id: String,
    pub template_item: String,
    pub description: Either<String, Description>,
    pub decal: i8,
}
