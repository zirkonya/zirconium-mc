use serde::{Deserialize, Serialize};
use zr_binary_macros::Binary;

#[derive(Binary, Serialize, Deserialize)]
pub struct BannerPattern {
    pub asset_id: String,
    pub translation_key: String,
}
