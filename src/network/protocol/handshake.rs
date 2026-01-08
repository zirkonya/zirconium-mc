use std::sync::Arc;

use crate::network::{
    context::State,
    payload::{Payload, handshake::Handshake},
    protocol::{InnerProtocol, action::Action},
};

#[derive(Default, Clone)]
pub struct HandshakeProtocol;

impl InnerProtocol for HandshakeProtocol {
    fn handle_message(&self, payload: Payload) -> Arc<[Action]> {
        match payload {
            Payload::Handshake(Handshake {
                protocol_version: _,
                server_addr: _,
                server_port: _,
                intent,
            }) => {
                // TODO : Hanshake test protocol_version
                // TODO : Maybe use server_addr & server port
                let next_state = match *intent.value() {
                    1 => State::Status,
                    2 => State::Login,
                    _ => unreachable!(),
                };
                [
                    Action::Log("🤝 Handshake".to_string().into()),
                    Action::UpdateState(next_state),
                    Action::Log(format!("next_state={next_state:?}").into()),
                ]
                .into()
            }
            _ => Arc::new([]),
        }
    }
}
