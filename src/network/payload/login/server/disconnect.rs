use std::sync::Arc;
use zr_protocol::macros::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct Disconnect {
    reason: Arc<[u8]>,
}
