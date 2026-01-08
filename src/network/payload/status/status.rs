use std::{path::Path, sync::Arc};

use base64::{Engine, prelude::BASE64_STANDARD};
use zr_protocol::{macros::Serialize, message::size::PrefixSize, serialization::Serialize};

use crate::utils::varint::VarInt;

#[derive(Debug, Serialize)]
pub struct StatusRequest;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Version {
    name: String,
    protocol: i32,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Player {
    max: i32,
    online: i32,
    sample: Vec<(String, String)>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Description {
    text: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonField {
    version: Version,
    players: Player,
    description: Description,
    favicon: Option<String>,
    enforce_secure_chat: Option<bool>,
}

impl JsonField {
    pub fn add_favicon<P: AsRef<Path>>(mut self, image: P) -> Self {
        let bytes = std::fs::read(image).ok();
        self.favicon =
            bytes.map(|v| format!("data:image/png;base64,{}", BASE64_STANDARD.encode(v)));
        self
    }
}

impl Serialize for JsonField {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        serde_json::to_writer(buffer, self).map_err(std::io::Error::other)
    }

    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        serde_json::from_reader(reader).map_err(std::io::Error::other)
    }
}

impl Default for JsonField {
    fn default() -> Self {
        Self {
            version: Version {
                name: String::from("zirconium-mc"),
                protocol: 773,
            },
            players: Player {
                max: 128,
                online: -16,
                sample: Vec::new(),
            },
            description: Description {
                text: "Hello there".to_string(),
            },
            favicon: None,
            enforce_secure_chat: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct StatusResponse {
    pub json_response: PrefixSize<VarInt<i32>, Arc<[u8]>>,
}
