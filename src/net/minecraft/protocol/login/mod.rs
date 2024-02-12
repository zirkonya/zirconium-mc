use crate::{gen_bin, gen_struct, format::uuid::Uuid, tools::{maths::varint::varint::VarInt, utils::bin::binarray::Array}};
// C -> S
gen_bin!(LoginStartPacket { name: String, player_uuid: Uuid });
gen_bin!(EncryptionResponsePacket { shared_secret: Vec<u8>, verify_token: Vec<u8> });
gen_bin!(LoginPluginResponsePacket { message_id: VarInt<i32>, successful: bool, data: Vec<u8> });
gen_bin!(LoginAcknowledgePacket { empty: () });

// S -> C
gen_bin!(DisconnectPacket { json: String });
gen_bin!(EncryptionRequestPacket { server_id: String, public_key: Vec<u8>, verify_token: Vec<u8> });

gen_bin!(Property { name: String, value: String, is_signed: bool, signature: Option<String> });

gen_bin!(LoginSuccessPacket { uuid: Uuid, username: String, properties: Array<Property> });