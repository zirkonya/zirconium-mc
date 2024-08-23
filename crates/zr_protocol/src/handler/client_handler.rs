use std::{collections::HashMap, fs, sync::{Arc, Mutex, RwLock}};

use std::net::TcpStream;
use zr_binary::binary::Binary;
use zr_binary::varint::VarInt;
use zr_nbt::Nbt;
use zr_network::packet::{Packet, PacketData};
use zr_network::handler::Handler;
use zr_network::handler::client::{sender::ClientSenderThread, receiver::ClientReceiverThread};
use zr_network::handler::options::HandlerOptions;
use zr_network::handler::client::ClientNetworkHandler;
use zr_network::handler::error::{NetworkError, Result};
use crate::protocol::{config::{self, registry::registry::Registry}, handshake, login, status};
use super::user::User;

use std::time::Duration;

#[derive(Debug)]
pub enum State {
    Handshake,
    Status,
    Login,
    Config,
    Play
}

pub struct ClientProtocolHandler {
    client_handler: Arc<Mutex<ClientNetworkHandler<ClientSenderThread, ClientReceiverThread>>>,
    alive: Arc<RwLock<bool>>,
    state: State,
    user: User,
}

macro_rules! lock {
    ($handler: expr) => {
        $handler.lock().map_err(|_| NetworkError::ConnectionClose)
    };
}

impl ClientProtocolHandler {
    pub fn new(stream: TcpStream) -> Self {
        Self::with_options(stream, HandlerOptions::default())
    }

    pub fn with_options(stream: TcpStream, options: HandlerOptions) -> Self {
        let alive = Arc::new(RwLock::new(true));
        let handler = Arc::new(Mutex::new(ClientNetworkHandler::with_options(stream, alive.clone(), options)));
        Self { client_handler: handler, alive, state: State::Handshake, user: User::new() }
    }

    fn read_packet(&mut self) -> Result<Option<Packet>> {
        let mut handler = lock!(self.client_handler)?;
        Ok(handler.read_packet())
    }

    fn write<P: PacketData + Binary>(&mut self, packet: P) -> Result<()> {
        let mut handler = lock!(self.client_handler)?;
        handler.write(packet);
        Ok(())
    }

    fn handshake(&mut self) -> Result<()> {
        let mut handler = lock!(self.client_handler)?;
        if let Some(handshake) = handler.read::<handshake::Handshake>() {
            println!("DEBUG {handshake:?}");
            match handshake.next_state.0 {
                1 => self.state = State::Status,
                2 => self.state = State::Login,
                x => return Err(NetworkError::Custom(format!("expected 1 or 2, found {x}")))
            }
        } else {
            println!("no handhake");
        }
        Ok(())
    }

    fn status(&mut self) -> Result<()> {
        // TODO : timeout
        let mut count = 0;
        loop {
            if let Some(packet) = self.read_packet()? {
                match packet.id() {
                    status::client::Ping::ID => {
                        println!("DEBUG ping");
                        let ping = packet.data::<status::client::Ping>().map_err(|error| NetworkError::PacketError(error))?;
                        self.write::<status::server::Pong>(status::server::Pong { payload: ping.payload })?;
                        count += 1;
                    },
                    status::client::StatusRequest::ID => {
                        println!("DEBUG Status");
                        const JSON: &str = "{\"version\":{\"name\":\"1.20.4\",\"protocol\":765},\"players\":{\"max\":20,\"online\":1,\"sample\":[{\"name\":\"zirkonya\",\"id\":\"2bf12816-3494-48de-b69f-95662f7d34c1\"}]},\"description\":{\"text\":\"My rusty server\"},\"favicon\":\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFAUlEQVR4nO1bPWskRxB9NmYEEpNdcsKgTFrBmQM1GJQNLM6sZDOBIwWHcl12sTP/AKFA0cFll1jpwWQHhhEICyxtsODESg4UDCPYSexgr0c1PdWf07MbeB8I7Ud/vVfV1TXdvcAaa6zxf8Y3q+z84ODgX91319fXSxnb0gQwkTVhaCGiN763t9ciurW11bvNIUWI2rBKXkUfMYYSIVqjNvIUoUIMIUKUBn3Im+AiTGwRejcWi7wKmxixhAhuxJd4Xdfa75IkYT9fhkcEVXYhbyLMoY8IQLgQ34ZUMqGu62jkAaCqKqc2QvMMb9V01vclDQCz2azz2Wg0YssO5QlehTnyvsQ50jqoYgwhgnPBUPKXl5et91mWuXbZwmg08sofXEUIjgEhLg8AeZ4jz3Pvend3d0H92eCkkmr9EMvb4OoZQogmMMbIFaILoCM+vdK3v/vz82sXIWhsMIkQRYC+5CnxzY0UT/PS2N/mRgoATbk3v2VsOSlCXy/wigG+8161OiUvidpw8TbHxdvcq18fGAWg1reRV60/vdKTPBRZxxM2N9Km/HgiOnXevyvw/l3RvJdBsaoq52SJw3fBNb/C5PY6d/9c5Ozn44nAlynfz9O8dPYaH2jnh8vct815DtTKv3/IXcbYqfvLrwsPyfMcp6enrTJcTDDFgajPAjry1HLSK75MgaPjzKldzvIhuQQHVoAQ60vydC5LPM1LHIqs9f7mtsCnjwVcIEWT/2ks6IsgDzAlOXKQlDDAz3vbqmCa87ap5oqOACE7PNxyd3NbGAmoAZILmJxAarnz83Pr+EyPylYPUN3fJ8XllrpQ2BKoUPQOgjpXtFmUQsYN+qeCBswQL9ChVx7gsuTRwapxoS6Bv/7mAxqXNh+KTJtDhCLaMqhb0qRFxW6GWnGAJAX2dwT2d7qZ3+tXoql7dJzh08cCL3b5vk9OTprXvplhFAGOjrNOBmea7z8cdz9Thbi5XXiGzA7HE9EsmzEzwt6pMIBORqcOkBKrS+DPD/q29ndEa1o0ucLtcxk6Naj1Q+DtAbYOTeQl6hKd6WCro4LuIfSBVQBuy9okwngiMJ6IJl83gYqwTYon6cLK3KpBVwnTOFxjQfRzgfJh8X96tXhoSbfd6v2jyW6pCK9f2UX1RS8BVDeUlikfgHkJPHwlpYog3rQtTpEY4psMjD6oqsr4NOgUBJMkcdoN+vHwmdWL3WdvUFFcaPrRkFf3Ar7/KV5WGOwB6vxTg59KPt3uegL9jpI3JTuSvOw/SRLj0ZoNUZZBn3XZFhNMTankY4CdG67nf7PZrHk2lwnLRgqkL/Xuz0HGOZ3laaxRd4BsuL+/j7crzEEudzRV5cinL/nXPuSB8BMpHZz3BHUDoIeddKeGBsR5ybv2H5/bUV09C1C3vajru8x7m/UBy8GIy4GoetqrbldRIShM5Ln9PnXe2wRwIQ8EnAwB/iK4gGaOVICzszMAwOPjI4B4xCWCYoA6CCHaVnZJg7ny6smxJK/rNwZ63Q8AuvvwRdG1vskjKHkJlTjgdlXG1/rAAAJIcELosCryQIQrMkA/ESR5jjhgJx9KXCKKAID9mJoTQ40dFENanSLaLTGJvrfDffbzViKARGwhfI+4Y5AHlnBX2EWIVZEHVnBbnAoScrEhJnlgRb8XCEFs4hJL/cVICIYiLjH4b4ZCMTRxiUE7CRFjWcTXWGONNQDgP+DcfkNgWGhTAAAAAElFTkSuQmCC\",\"enforcesSecureChat\":false,\"previewsChat\":false}";
                        self.write::<status::server::StatusResponse>(status::server::StatusResponse { json_response: JSON.to_string() })?;
                        count += 1;
                    }
                    id => return Err(NetworkError::Custom(format!("Wrong packet id expected 0x{:02x} or 0x{:02x} but found 0x{:02x}", status::server::Pong::ID, status::server::StatusResponse::ID, id)))
                }
            }
            if count >= 2 { break; }
        }
        Ok(())
    }

    fn get_registry_data_codec(&self) -> HashMap<String, Registry> {
        let raw = fs::read_to_string("./1.20.2.json").unwrap();
        serde_json::from_str(&raw).unwrap()
    }

    fn config(&mut self) -> Result<()> {
        if let Some(packet) = self.read_packet()? {
            self.write(config::server::RegistryData { registry_codec: Nbt::new(self.get_registry_data_codec()) })?;
            match packet.id() {
                config::client::AcknowledgeFinishConfiguration::ID => self.state = State::Play,
                _ => panic!("Config")
            }
        }
        Ok(())
    }

    fn set_compression(&mut self) -> Result<()> {
        let is_compression_set = {
            let handler = lock!(self.client_handler)?;
            handler.get_options().is_compression_set()
        };
        if is_compression_set {
            let threshold: VarInt<i32> = {
                let handler = lock!(self.client_handler)?;
                handler.get_options().threshold().unwrap_or_default().into()
            };
            self.write(login::server::SetCompression { threshold })?;
            {
                let mut handler = lock!(self.client_handler)?;
                handler.get_options_mut().enable_compression();
            }
        }
        Ok(())
    }

    fn login_success(&mut self) -> Result<()> {
        self.write(login::server::LoginSuccess { uuid: self.user.uuid.unwrap(), username: self.user.username.clone().unwrap(), properties: Vec::new() })?;
        Ok(())
    }

    fn login(&mut self) -> Result<()> {
        loop {
            if let Some(packet) = self.read_packet()? {
                match packet.id() {
                    login::client::LoginStart::ID => {
                        let login::client::LoginStart { name, player_uuid  } = login::client::LoginStart::from_packet(packet).map_err(|error| NetworkError::PacketError(error))?;
                        self.user.username  = Some(name);
                        self.user.uuid      = Some(player_uuid);
                        let is_offline = {
                            let handler = self.client_handler.lock().map_err(|_| NetworkError::ConnectionClose)?;
                            handler.get_options().is_offline()
                        };
                        if is_offline {
                            self.set_compression()?;
                            self.login_success()?;
                        } else {
                            todo!("Cipher");
                        }
                    },
                    login::client::EncryptionResponse::ID => {
                        self.set_compression()?;
                        self.login_success()?;
                    },
                    login::client::LoginAcknowledge::ID => {
                        return Ok(());
                    }
                    _ => panic!()
                }
            }
        }
    }

    fn play(&mut self) -> Result<()> {
        todo!()
    }
}

impl Handler for ClientProtocolHandler {
    fn on_enable(handler: Arc<Mutex<Self>>) -> Result<()> {
        let handler = handler.lock().map_err(|_| NetworkError::ConnectionClose)?;
        Handler::run(handler.client_handler.clone());
        std::thread::sleep(Duration::from_secs(5));
        println!("START ! ");
        Ok(())
    }

    fn on_update(handler: Arc<Mutex<Self>>) -> Result<()> {
        let mut handler = handler.lock().map_err(|_| NetworkError::ConnectionClose)?;
        match &handler.state {
            State::Handshake => handler.handshake()?,
            State::Status =>    handler.status()?,
            State::Config =>    handler.config()?,
            State::Login =>     handler.login()?,
            State::Play =>      handler.play()?
        }
        Ok(())
    }

    fn on_disable(_: Arc<Mutex<Self>>) -> Result<()> {
        Ok(())
    }

    fn stop(&mut self) {
        *self.alive.write()
            .unwrap_or_else(|mut e| {
                **e.get_mut() = false;
                self.alive.clear_poison();
                e.into_inner()
            }) = false;
    }

    fn is_alive(&mut self) -> bool {
        *self.alive.read().unwrap()
    }

}
