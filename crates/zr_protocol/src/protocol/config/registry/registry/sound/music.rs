use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Music {
    pub replace_current_music: i8,
    pub max_delay: i32,
    pub sound: String,
    pub min_delay: i32,
}

