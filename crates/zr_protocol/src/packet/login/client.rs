use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_network_macros::Packet;
use uuid::Uuid;

#[derive(Binary, Packet)]
#[id = 0x00]
pub struct LoginStart {
    pub(crate) name: String,
    pub(crate) player_uuid: Uuid,
}

#[derive(Binary, Packet)]
#[id = 0x01]
pub struct EncryptionResponse {
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) shared_secret: Vec<u8>,
    #[prefixed_length = "VarInt<i32>"]
    pub(crate) verify_token: Vec<u8>,
}

#[derive(Binary, Packet)]
#[id = 0x02]
pub struct LoginPluginResponse {
    pub(crate) message_id: VarInt<i32>,
    successful: bool,
    #[some_if = "self.successful"]
    pub(crate) data: Option<Vec<u8>>,
}

#[derive(Binary, Packet)]
#[id = 0x03]
pub struct LoginAcknowledge;

