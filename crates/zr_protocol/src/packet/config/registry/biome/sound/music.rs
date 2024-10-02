use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Music {
    sound: String,
    min_delay: i32,
    max_delay: i32,
    replace_current_music: i8,
}
