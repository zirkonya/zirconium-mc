use std::sync::Arc;
use crate::network::{
    payload::Payload,
    protocol::{InnerProtocol, action::Action},
};

#[derive(Clone, Default)]
pub struct PlayProtocol;

impl InnerProtocol for PlayProtocol {
    // TODO : Add context
    fn handle_message(&self, payload: Payload) -> Arc<[Action]> {
        match payload {
            Payload::ClientTickEnd(_) => [Action::Log("Got tick".into())].into(), // MAYBE calcul TPS
            _ => [].into(),
        }
    }
}
