use crate::{gen_bin, gen_struct, tools::maths::varint::varint::VarInt, net::minecraft::State};

gen_bin!(HandshakePacket { protocol: VarInt<i32>, server_addr: String, server_port: u16, next_state: State });
