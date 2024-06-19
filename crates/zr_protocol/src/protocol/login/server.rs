use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_data_type::string::Identifier;
use zr_network_macros::Packet;

pub type Uuid = u128;

#[derive(Binary, Packet)]
#[id = 0x00]
pub struct Disconnect {
    pub reason: String,
}

#[derive(Binary, Packet)]
#[id = 0x01]
pub struct EncryptionRequest {
    pub(crate) server_id: String,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) public_key: Vec<u8>,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) verify_token: Vec<u8>,
}

#[derive(Debug, Binary, Clone, Default)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub is_signed: bool,
    #[some_if = "self.is_signed"]
    pub signature: Option<String>,
}

#[derive(Binary, Packet)]
#[id = 0x02]
pub struct LoginSuccess {
    pub(crate) uuid: Uuid,
    pub(crate) username: String,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) properties: Vec<Property>,
}

#[derive(Binary, Packet)]
#[id = 0x03]
pub struct SetCompression {
    pub(crate) threshold: VarInt<i32>,
}

#[derive(Binary, Packet)]
#[id = 0x04]
pub struct LoginPluginRequest {
    pub(crate) message_id: VarInt<i32>,
    pub(crate) channel: Identifier,
    pub(crate) data: Vec<u8>,
}