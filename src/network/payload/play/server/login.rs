use crate::utils::{identifier::Identifier, varint::VarInt};
use std::sync::Arc;
use zr_protocol::{macros::Serialize, message::size::PrefixSize};

#[derive(Serialize, Debug)]
pub struct LoginPlay {
    entity_id: i32,
    is_hardcore: bool,
    dimension_names: PrefixSize<VarInt<i32>, Arc<[Identifier]>>,
    max_players: VarInt<i32>,
    view_distance: VarInt<i32>,
    simulation_distance: VarInt<i32>,
    reduces_debug_info: bool,
    enable_respawn_screen: bool,
    do_limited_crafting: bool,
    dimension_type: VarInt<i32>,
    dimension_name: Identifier,
    hashed_seed: i64,
    gamemode: u8,
    previous_gamemode: i8,
    is_debug: bool,
    is_flat: bool,
    has_death_location: bool,
    // #[skip] // TODO : if (has_death_location)
    death_dimension_name: (), //Option<PrefixSize<VarInt<i32>, Arc<[u8]>>>, // TODO : Identifier
    // #[skip] // TODO : if (has_death_location)
    death_location: (),
    portal_cooldown: VarInt<i32>,
    sea_level: VarInt<i32>,
    enforce_secure_chat: bool,
}

impl Default for LoginPlay {
    fn default() -> Self {
        Self {
            entity_id: 0,
            is_hardcore: false,
            dimension_names: PrefixSize::new([Identifier::from("minecraft:overworld")].into()),
            max_players: VarInt::new(256),
            view_distance: VarInt::new(10),
            simulation_distance: VarInt::new(10),
            reduces_debug_info: false,
            enable_respawn_screen: true,
            do_limited_crafting: false,
            dimension_type: VarInt::new(0),
            dimension_name: Identifier::from("minecraft:overworld"), // PrefixSize::new(b"".to_owned().into()),
            hashed_seed: 0,
            gamemode: 1,
            previous_gamemode: 1,
            is_debug: false,
            is_flat: false,
            has_death_location: false,
            death_dimension_name: (),
            death_location: (),
            portal_cooldown: VarInt::new(0),
            sea_level: VarInt::new(64),
            enforce_secure_chat: false,
        }
    }
}
