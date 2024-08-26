use std::collections::HashMap;

use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_data_type::string::Identifier;
use zr_nbt::Nbt;
use zr_network_macros::Packet;

use super::registry::registry::Registry;

#[derive(Binary, Packet)]
#[id = 0x00]
pub struct ClientboundPluginMessage {
    pub(crate) channel: Identifier,
    pub(crate) data: Vec<u8>,
}

#[derive(Binary, Packet)]
#[id = 0x01]
pub struct Disconnect {
    pub(crate) reason: String,
}

#[derive(Binary, Packet)]
#[id = 0x02]
pub struct FinishConfiguration;

#[derive(Binary, Packet)]
#[id = 0x03]
pub struct ClientboundKeepAlive {
    pub(crate) keep_alive_id: i64,
}

#[derive(Binary, Packet)]
#[id = 0x04]
pub struct Ping {
    pub(crate) ping_id: i32,
}

// TODO : NBT
#[derive(Binary, Packet)]
#[id = 0x05]
pub struct RegistryData {
    pub(crate) registry_codec: Nbt<HashMap<String, Registry>>,
}

#[derive(Binary, Packet)]
#[id = 0x06]
pub struct RemoveResourcePack {
    pub(crate) has_uuid: bool,
    #[some_if = "self.has_uuid"]
    pub(crate) uuid: Option<u128>,
}

#[derive(Binary, Packet)]
#[id = 0x07]
pub struct AddResourcePack {
    pub(crate) uuid: u128,
    pub(crate) url: String,
    pub(crate) hash: String,
    pub(crate) forced: bool,
    pub(crate) has_prompt_message: bool,
    #[some_if = "self.has_prompt_message"]
    pub(crate) prompt_message: Option<String>, // Text component
}

#[derive(Binary, Packet)]
#[id = 0x08]
pub struct FeatureFlags {
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) feature_flags: Vec<Identifier>,
}

#[derive(Debug, Binary, Clone)]
pub struct Tag {
    pub(crate) tag_name: Identifier,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) entries: Vec<VarInt<i32>>,
}

#[derive(Binary, Packet)]
#[id = 0x09]
pub struct UpdateTags {
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) tags: Vec<Tag>,
}

