use std::sync::Arc;

use zr_protocol::{
    macros::Serialize,
    message::{option::PrefixOption, size::PrefixSize},
};

use crate::utils::{identifier::Identifier, varint::VarInt};

#[derive(Serialize, Debug, Clone)]
pub struct Entry {
    pub entry_id: Identifier,            // TODO : Identifier
    pub data: PrefixOption<Arc<[u8]>>, // TODO : NBT
}

#[derive(Serialize, Debug, Clone)]
pub struct RegistryData {
    pub registry_id: Identifier, // TODO : Identifier
    pub entries: PrefixSize<VarInt<i32>, Arc<[Entry]>>,
}
