use std::sync::Arc;

use crate::network::{context::State, payload::Payload};

#[derive(Debug)]
pub enum Action {
    SendBundle(Arc<[Payload]>),
    SendMessage(Payload),
    BroadcastMessage(Payload),
    BroadcastExceptSelfMessage(Payload),
    UpdateState(State),
    SetCompression(i32),
    Log(Arc<str>),
}
