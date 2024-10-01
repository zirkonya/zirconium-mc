use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

use serde::Serialize;
// ---
use uuid::Uuid;
use zr_network::packet::{packet::Packet, PacketData};

use crate::packet::status::{
    client::{Ping, StatusRequest},
    server::{Pong, StatusResponse},
};

use super::{
    handler::{Next, PacketHandler},
    protocol_handler::Client,
};

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonResponse {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    pub favicon: Option<String>,
    pub enforces_secure_chat: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    pub max: u32,
    pub online: u32,
    pub sample: Vec<Sample>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub name: String,
    pub id: Uuid,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub text: String,
}
// ---

pub struct StatusProtocol {
    json_response: Arc<RwLock<JsonResponse>>,
}

impl StatusProtocol {
    pub fn new(json_response: Arc<RwLock<JsonResponse>>) -> Self {
        Self { json_response }
    }

    fn on_ping_request(&self, payload: i64) -> Next {
        Next::SendPacket(Pong { payload }.to_packet())
    }

    fn on_status_request(&self) -> Next {
        match self.json_response.read() {
            Ok(json_response) => {
                let json_response = serde_json::to_string(json_response.deref()).unwrap();
                Next::SendPacket(StatusResponse { json_response }.to_packet())
            }
            Err(err) => {
                eprintln!("{err:?}");
                Next::Disconnect
            }
        }
    }
}

impl PacketHandler for StatusProtocol {
    fn handle_packet(&mut self, _: &mut Client, packet: Packet) -> Next {
        match packet.id() {
            Ping::ID => {
                let Ping { payload } = Ping::from_packet(packet).unwrap();
                self.on_ping_request(payload)
            }
            StatusRequest::ID => self.on_status_request(),
            _ => Next::Disconnect,
        }
    }
}
