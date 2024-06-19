use serde::{Deserialize, Serialize};

use self::{
    armor_trim::{material::ArmorTrimMaterial, pattern::ArmorTrimPattern},
    biome::Biome,
    chat::Chat,
    damage::Damage,
    dimension::Dimension,
};

pub mod armor_trim;
pub mod biome;
pub mod chat;
pub mod damage;
pub mod dimension;
pub mod particle;
pub mod sound;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Registry {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: Vec<RegistryEntry>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub name: String,
    pub id: i32,
    pub element: Entry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Entry {
    None,
    ArmorTrimMaterial(ArmorTrimMaterial),
    ArmorTrimPattern(ArmorTrimPattern),
    Biome(Biome),
    Chat(Chat),
    Damage(Damage),
    Dimension(Dimension),
}

impl Default for Entry {
    fn default() -> Self {
        Self::None
    }
}

#[cfg(test)]
mod tests {
    use zr_nbt::either::Either;

    use crate::protocol::config::registry::registry::Registry;
    use super::{armor_trim::pattern::ArmorTrimPattern, Entry, RegistryEntry};

    #[test]
    pub fn test_registry_to_json() {
        let registry = Registry {
            type_field: "type/test".to_string(),
            value: vec![RegistryEntry {
                name: "test".to_string(),
                id: 0,
                element: Entry::ArmorTrimPattern(ArmorTrimPattern {
                    asset_id: "asset_id".to_string(),
                    template_item: "template_item".to_string(),
                    description: Either::One("description".to_string()),
                    decal: 3,
                }),
            }],
        };
        let json = serde_json::to_string(&registry).expect("Cannot parse to json");
        let reverse: Registry = serde_json::from_str(&json).expect("Cannot read json");
        assert_eq!(registry, reverse)
    }
}
