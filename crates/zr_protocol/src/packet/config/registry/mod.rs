use std::collections::HashMap;

use armor_trim::{material::ArmorTrimMaterial, pattern::ArmorTrimPattern};
use banner_pattern::BannerPattern;
use biome::Biome;
use chat_type::ChatType;
use damage_type::DamageType;
use dimension_type::DimensionType;
use painting_variant::PaintingVariant;
use serde::{Deserialize, Serialize};
use wolf_variant::WolfVariant;

pub mod armor_trim;
pub mod banner_pattern;
pub mod biome;
pub mod chat_type;
pub mod damage_type;
pub mod dimension_type;
pub mod painting_variant;
pub mod wolf_variant;

#[derive(Clone, Serialize, Deserialize)]
pub struct Registry {
    #[serde(rename = "minecraft:trim_material")]
    armor_trim_material: HashMap<String, ArmorTrimMaterial>,
    #[serde(rename = "minecraft:trim_pattern")]
    armor_trim_pattern: HashMap<String, ArmorTrimPattern>,
    #[serde(rename = "minecraft:banner_pattern")]
    banner_pattern: HashMap<String, BannerPattern>,
    #[serde(rename = "minecraft:worldgen/biome")]
    biome: HashMap<String, Biome>,
    #[serde(rename = "minecraft:chat_type")]
    chat_type: HashMap<String, ChatType>,
    #[serde(rename = "minecraft:damage_type")]
    damage_type: HashMap<String, DamageType>,
    #[serde(rename = "minecraft:dimension_type")]
    dimension_type: HashMap<String, DimensionType>,
    #[serde(rename = "minecraft:wolf_variant")]
    wolf_variant: HashMap<String, WolfVariant>,
    #[serde(rename = "minecraft:painting_variant")]
    painting_variant: HashMap<String, PaintingVariant>,
}

#[cfg(test)]
pub mod test {
    use zr_nbt::Nbt;

    use super::Registry;

    #[test]
    pub fn test_registry() {
        let json =
            include_str!("/home/zirkonya/Desktop/workspace/rust/minecraft-server/1.20.6.json");

        let nbt: Nbt<Registry> = serde_json::from_str::<Nbt<Registry>>(json).unwrap();
        let json_2 = serde_json::to_string(&nbt).unwrap();
        println!("{json_2}");
        assert!(false);
    }
}
