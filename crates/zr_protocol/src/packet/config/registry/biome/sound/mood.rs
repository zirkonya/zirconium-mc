use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MoodSound {
    sound: String,
    tick_delay: i32,
    block_search_extent: i32,
    offset: f64,
}
