use zr_binary::varint::VarInt;
use zr_binary_macros::Binary;
use zr_data_type::string::Identifier;
use zr_network_macros::Packet;

#[derive(Binary, Packet)]
#[id = 0x00]
pub struct ClientInformation {
    pub(crate) local: String,
    pub(crate) view_distance: i8,
    pub(crate) chat_mod: VarInt<i32>, 
    pub(crate) chat_color: bool,
    pub(crate) display_skin_part: u8,
    pub(crate) main_hand: VarInt<i32>, 
    pub(crate) enable_text_filtering: bool,
    pub(crate) allow_server_listing: bool,
}

#[derive(Binary, Packet)]
#[id = 0x01]
pub struct ServerboundPluginMessage {
    pub(crate) channel: Identifier,
    pub(crate) data: Vec<u8>,
}

#[derive(Binary, Packet)]
#[id = 0x02]
pub struct AcknowledgeFinishConfiguration;

#[derive(Binary, Packet)]
#[id = 0x03]
pub struct ServerboundKeepAlive {
    pub(crate) keep_alive_id: i64,
}

#[derive(Binary, Packet)]
#[id = 0x04]
pub struct Pong {
    pub(crate) pong_id: i32,
}

#[derive(Binary, Packet)]
#[id = 0x05]
pub struct ResourcePackResponse {
    pub(crate) uuid: u128,
    pub(crate) result: VarInt<i32>,
}
