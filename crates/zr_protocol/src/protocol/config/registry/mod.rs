use serde::{Deserialize, Serialize};

pub mod registry;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub color: Option<String>,
    pub translate: String,
}