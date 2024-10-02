// TODO : TextFormating
pub type TextFormating = String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Decoration {
    translation_key: String,
    style: Option<TextFormating>,
    parameters: Vec<String>,
}
