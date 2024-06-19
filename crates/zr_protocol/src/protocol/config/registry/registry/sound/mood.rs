use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoodSound {
    pub tick_delay: i32,
    pub offset: f64,
    pub sound: String,
    pub block_search_extent: i32,
}

