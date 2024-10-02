use decoration::Decoration;
use serde::{Deserialize, Serialize};

pub mod decoration;

#[derive(Clone, Serialize, Deserialize)]
pub struct ChatType {
    chat: Decoration,
    narration: Decoration,
}
