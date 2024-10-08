use serde::{Deserialize, Serialize};
use zr_nbt::either::Either;

use super::{
    particle::Particle,
    sound::{additions::AdditionsSound, ambient::AmbientSound, mood::MoodSound, music::Music},
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Effects {
    fog_color: i32,
    water_color: i32,
    water_fog_color: i32,
    sky_color: i32,
    foliage_color: Option<i32>,
    grass_color: Option<i32>,
    grass_color_modifier: Option<String>,
    particle: Option<Particle>,
    ambient_sound: Option<Either<String, AmbientSound>>,
    mood_sound: Option<MoodSound>,
    additions_sound: Option<AdditionsSound>,
    music: Option<Music>,
}
