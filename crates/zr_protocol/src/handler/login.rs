use super::{
    handler::{Next, PacketHandler},
    protocol_handler::{Client, State},
};
use crate::packet::{
    config::{self, registry::Registry},
    login::{client, server},
};
use rsa::{pkcs8::EncodePublicKey, RsaPrivateKey, RsaPublicKey};
use zr_binary::varint::VarInt;
use zr_core::entity::player::Player;
use zr_nbt::Nbt;
use zr_network::{error::network::NetworkError, packet::PacketData};

const RSA_KEY_BITS: usize = 1024;

pub struct LoginProtocol {
    key_pair: (RsaPrivateKey, RsaPublicKey),
    verify_token: Vec<u8>,
}

impl LoginProtocol {
    pub fn new() -> rsa::Result<Self> {
        Self::new_with_key_bits(RSA_KEY_BITS)
    }

    pub fn new_with_key_bits(key_bits: usize) -> rsa::Result<Self> {
        let mut rng = rand::thread_rng();
        let private = RsaPrivateKey::new(&mut rng, key_bits)?;
        let public = RsaPublicKey::from(private.clone());
        Ok(Self {
            key_pair: (private, public),
            verify_token: b"verify_token".to_vec(),
        })
    }

    pub fn public_key_der(&self) -> Result<Vec<u8>, rsa::pkcs8::spki::Error> {
        self.key_pair.1.to_public_key_der().map(|d| d.to_vec())
    }

    fn on_login_start(&self) -> Next {
        match self.public_key_der() {
            Ok(public_key) => Next::SendPacket(
                server::EncryptionRequest {
                    server_id: String::new(),
                    public_key,
                    verify_token: self.verify_token.to_vec(),
                }
                .to_packet(),
            ),
            Err(_) => Next::Disconnect,
        }
    }

    fn on_login_acknowledge(&self, client: &mut Client) -> Next {
        client.change_state(State::Config);
        Next::SendPackets(vec![
            config::server::KnownPacks { known_pack: vec![] }.to_packet(),
            config::server::RegistryData {
                // TODO : remove hard json file
                registry_codec: serde_json::from_str::<Nbt<Registry>>(include_str!(
                    "/home/zirkonya/Desktop/workspace/rust/minecraft-server/1.20.6.json"
                ))
                .unwrap(),
            }
            .to_packet(),
        ])
    }

    fn on_encryption_response(
        &self,
        client: &mut Client,
        player: Player,
    ) -> Result<Next, NetworkError> {
        let uuid = player.uuid();
        let username = player.name();
        client.send_packet(
            server::SetCompression {
                threshold: VarInt::new(2048),
            }
            .to_packet(),
        )?;

        client.active_compression();
        Ok(Next::SendPacket(
            server::LoginSuccess {
                uuid,
                username,
                properties: vec![],
                strict_error_handling: false,
            }
            .to_packet(),
        ))
    }

    fn wrong_packet_id(&self) -> Next {
        eprintln!("unknown packet id");
        Next::SendPacket(
            server::Disconnect {
                reason: "No hablo español".to_string(),
            }
            .to_packet(),
        )
    }
}

// TODO : complete login protocol
impl PacketHandler for LoginProtocol {
    fn handle_packet(
        &mut self,
        client: &mut Client,
        packet: zr_network::packet::packet::Packet,
    ) -> Next {
        match packet.id() {
            client::LoginStart::ID =>
            /*self.on_login_start()*/
            {
                let login_start = client::LoginStart::from_packet(packet).unwrap();
                client.player = Some(Player::new(login_start.player_uuid, login_start.name));
                self.on_encryption_response(client, client.player().clone().unwrap())
                    .unwrap()
            }
            client::EncryptionResponse::ID => {
                let player = client.player().clone().unwrap();
                self.on_encryption_response(client, player).unwrap()
            }
            client::LoginAcknowledge::ID => self.on_login_acknowledge(client),
            client::LoginPluginResponse::ID => Next::Wait,
            _ => self.wrong_packet_id(),
        }
    }
}
