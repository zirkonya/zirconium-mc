use crate::{gen_bin, gen_struct};

// S -> C
gen_bin!(StatusResponsePacket { json_response: String });
gen_bin!(PingResponsePacket { payload: u64 });

// C - S
gen_bin!(StatusRequestPacket);
gen_bin!(PingRequestPacket { payload: u64 });