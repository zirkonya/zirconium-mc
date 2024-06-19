use serde_derive::{Deserialize, Serialize};


pub type Style = ();

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Decoration {
    pub translation_key: String,
    pub style: Option<Style>,
    pub parameters: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chat {
    pub chat: Decoration,
    pub narration: Decoration,
}
