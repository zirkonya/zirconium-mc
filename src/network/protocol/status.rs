use std::sync::Arc;
use zr_protocol::{message::size::PrefixSize, serialization::Serialize};
use crate::network::{
    payload::{
        Payload,
        status::status::{JsonField, StatusResponse},
    },
    protocol::{InnerProtocol, action::Action},
};

#[derive(Default, Clone)]
pub struct StatusProtocol;

impl InnerProtocol for StatusProtocol {
    fn handle_message(&self, payload: Payload) -> Arc<[Action]> {
        match payload {
            Payload::StatusPingRequest(ping) => [
                Action::Log(format!("🏓 {}", ping.timestamp).into()),
                Action::SendMessage(Payload::StatusPingResponse(ping)),
            ]
            .into(),
            Payload::StatusRequest(_) => [
                Action::Log("🪧 Share status".into()),
                Action::SendMessage(Payload::StatusResponse(StatusResponse {
                    json_response: PrefixSize::new(
                        JsonField::default()
                            .add_favicon("./favicon.png")
                            .to_bytes()
                            .unwrap(),
                    ),
                })),
            ]
            .into(),
            _ => Arc::new([]),
        }
    }
}
