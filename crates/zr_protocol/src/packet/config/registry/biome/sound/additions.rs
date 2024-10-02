use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AdditionsSound {
    sound: String,
    tick_chance: f64,
}
