use serde::{Deserialize, Serialize};
use zr_nbt::either::Either;

use super::{particle::Particle, sound::{addition::AdditionsSound, ambient::AmbientSound, mood::MoodSound, music::Music}};


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Biome {
    pub has_precipitation: i8,
    pub temperature: f32,
    pub temperature_modifier: Option<String>,
    pub downfall: f32,
    pub effects: Effects,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Effects {
    pub fog_color: i32,
    pub water_color: i32,
    pub water_fog_color: i32,
    pub sky_color: i32,
    pub foliage_color: Option<i32>,
    pub grass_color: Option<i32>,
    pub grass_color_modifier: Option<String>,
    pub particle: Option<Particle>,
    pub music: Option<Music>,
    pub ambient_sound: Option<Either<String, AmbientSound>>,
    pub mood_sound: Option<MoodSound>,
    pub additions_sound: Option<AdditionsSound>,
}

