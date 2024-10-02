use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct PaintingVariant {
    asset_id: String,
    height: i32,
    width: i32,
}
