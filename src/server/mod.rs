use std::collections::HashMap;

use crate::{format::uuid::Uuid, minecraft::{player::Player, world::World}};

pub struct Server {
    worlds: HashMap<String, World>,
    players: HashMap<Uuid, Player>,
}