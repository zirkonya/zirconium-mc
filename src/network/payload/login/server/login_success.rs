use std::sync::Arc;

use zr_protocol::macros::Serialize;
use zr_protocol::message::{option::PrefixOption, size::PrefixSize};

use crate::utils::varint::VarInt;

type String = PrefixSize<VarInt<i32>, Arc<[u8]>>;

#[derive(Serialize, Debug, Clone)]
pub struct Profile {
    pub uuid: u128, // TODO : UUID
    pub username: PrefixSize<VarInt<i32>, Arc<[u8]>>,
    pub properties: PrefixSize<VarInt<i32>, Arc<[(String, String, PrefixOption<String>)]>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LoginSuccess {
    pub profile: Profile,
}
