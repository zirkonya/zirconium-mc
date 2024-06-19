use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}