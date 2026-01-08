use std::sync::Arc;
use zr_protocol::{
    macros::Serialize,
    message::{option::PrefixOption, size::PrefixSize},
};

use crate::utils::varint::VarInt;

#[derive(Serialize, Debug, Clone)]
pub struct CookieResponse {
    key: PrefixSize<VarInt<i32>, Arc<[u8]>>, // TODO : Identifier
    payload: PrefixOption<PrefixSize<VarInt<i32>, Arc<[u8]>>>,
}
