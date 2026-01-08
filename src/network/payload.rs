use crate::network::{
    context::{Bound, State},
    packet::Packet,
    payload::{
        config::{
            known_packs::KnownPacks,
            server::{finish_configuration::FinishConfiguration, registry_data::RegistryData},
        },
        handshake::*,
        login::{
            client::{
                cookie_response::*, encryption_response::*, login_acknowledged::*,
                login_plugin_response::*, login_start::*,
            },
            server::{
                cookie_request::*, disconnect::*, encryption_request::*, login_plugin_request::*,
                login_success::*, set_compression::*,
            },
        },
        play::{
            client::{client_tick_end::ClientTickEnd, confirm_teleportation::ConfirmTeleportation},
            server::{
                bundle_delimiter::BundleDelimiter, chunk_batch_start::ChunkBatchStart,
                chunk_data_and_update_light::ChunkDataAndUpdateLight, entity_event::EntityEvent,
                game_event::GameEvent, login::LoginPlay, recipe_book_settings::RecipeBookSettings,
                set_entity_metadata::SetEntityMetadata, set_held_item::SetHeldItem,
                synchronize_player_position::SynchronizePlayerPosition,
                update_entity_position::UpdateEntityPosition, update_recipes::UpdateRecipes,
            },
        },
        status::{ping::*, status::*},
    },
};
#[cfg(debug_assertions)]
use std::sync::Arc;
use zirconium_mc_macros::Packets;
use zr_protocol::serialization::Serialize;

pub mod config;
pub mod handshake;
pub mod login;
pub mod play;
pub mod status;

// TODO : add default payload "not yet implemented" for debug ! (DEBUG ONLY !)

#[cfg(debug_assertions)]
#[derive(zr_protocol::macros::Serialize, Debug)]
pub struct RawData {
    data: Arc<[u8]>,
}

#[derive(zr_protocol::macros::Serialize, Debug)]
pub struct Empty;

#[derive(Debug, Packets)]
pub enum Payload {
    /* HANDSHAKE */
    #[packet(id = 0x00, bound = Bound::Server,state = State::Handshaking)]
    Handshake(Handshake),
    /* STATUS */
    #[packet(id = 0x00, bound = Bound::Server, state = State::Status)]
    StatusRequest(StatusRequest),
    #[packet(id = 0x01, bound = Bound::Server, state = State::Status)]
    StatusPingRequest(StatusPing),
    #[packet(id = 0x00, bound = Bound::Client, state = State::Status)]
    StatusResponse(StatusResponse),
    #[packet(id = 0x01, bound = Bound::Client, state = State::Status)]
    StatusPingResponse(StatusPing),
    /* LOGIN */
    #[packet(id = 0x00, bound = Bound::Client, state = State::Login)]
    Disconnect(Disconnect),
    #[packet(id = 0x01, bound = Bound::Client, state = State::Login)]
    EncryptionRequest(EncryptionRequest),
    #[packet(id = 0x02, bound = Bound::Client, state = State::Login)]
    LoginSuccess(LoginSuccess),
    #[packet(id = 0x03, bound = Bound::Client, state = State::Login)]
    SetCompression(SetCompression),
    #[packet(id = 0x04, bound = Bound::Client, state = State::Login)]
    LoginPluginRequest(LoginPluginRequest),
    #[packet(id = 0x05, bound = Bound::Client, state = State::Login)]
    CookieRequest(CookieRequest),
    #[packet(id = 0x00, bound = Bound::Server, state = State::Login)]
    LoginStart(LoginStart),
    #[packet(id = 0x01, bound = Bound::Server, state = State::Login)]
    EncryptionResponse(EncryptionResponse),
    #[packet(id = 0x02, bound = Bound::Server, state = State::Login)]
    LoginPluginResponse(LoginPluginResponse),
    #[packet(id = 0x03, bound = Bound::Server, state = State::Login)]
    LoginAcknowledge(LoginAcknowledge),
    #[packet(id = 0x04, bound = Bound::Server, state = State::Login)]
    CookieResponse(CookieResponse),
    /* CONFIG */
    #[packet(id = 0x03, bound = Bound::Client, state = State::Config)]
    FinishConfiguration(FinishConfiguration),
    #[packet(id = 0x07, bound = Bound::Client, state = State::Config)]
    RegistryData(RegistryData),
    #[packet(id = 0x0E, bound = Bound::Client, state = State::Config)]
    ClientBoundKnownPack(KnownPacks),
    #[packet(id = 0x03, bound = Bound::Server, state = State::Config)]
    AcknowledgeFinishConfiguration(Empty),
    #[packet(id = 0x07, bound = Bound::Server, state = State::Config)]
    ServerBoundKnownPack(KnownPacks),
    /* PLAY */
    // SERVER
    #[packet(id = 0x00, bound = Bound::Client, state = State::Play)]
    BundleDelimiter(BundleDelimiter),
    #[packet(id = 0x0c, bound = Bound::Client, state = State::Play)]
    ChunkBatchStart(ChunkBatchStart),
    #[packet(id = 0x22, bound = Bound::Client, state = State::Play)]
    EntityEvent(EntityEvent),
    #[packet(id = 0x26, bound = Bound::Client, state = State::Play)]
    GameEvent(GameEvent),
    #[packet(id = 0x2c, bound = Bound::Client, state = State::Play)]
    ChunkDataAndUpdateLight(ChunkDataAndUpdateLight),
    #[packet(id = 0x30, bound = Bound::Client, state = State::Play)]
    LoginPlay(LoginPlay),
    #[packet(id = 0x33, bound = Bound::Client, state = State::Play)]
    UpdateEntityPosition(UpdateEntityPosition),
    #[packet(id = 0x46, bound = Bound::Client, state = State::Play)]
    SynchronizePlayerPosition(SynchronizePlayerPosition),
    #[packet(id = 0x4a, bound = Bound::Client, state = State::Play)]
    RecipeBookSettings(RecipeBookSettings),
    #[packet(id = 0x61, bound = Bound::Client, state = State::Play)]
    SetEntityMetadata(SetEntityMetadata),
    #[packet(id = 0x67, bound = Bound::Client, state = State::Play)]
    SetHeldItem(SetHeldItem),
    #[packet(id = 0x83, bound = Bound::Client, state = State::Play)]
    UpdateRecipes(UpdateRecipes),
    // CLIENT
    #[packet(id = 0x00, bound = Bound::Server, state = State::Play)]
    ConfirmTeleportation(ConfirmTeleportation),
    #[packet(id = 0x0c, bound = Bound::Server, state = State::Play)]
    ClientTickEnd(ClientTickEnd),
    #[cfg(debug_assertions)]
    #[default]
    NotYetImplemented(RawData),
}
