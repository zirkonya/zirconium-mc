use zr_protocol::macros::Serialize;

#[derive(Debug, Serialize)]
pub struct StatusPing {
    pub timestamp: u64,
}
