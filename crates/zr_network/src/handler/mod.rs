pub mod error;
pub mod options;
mod read_varint;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use zr_binary::{binary::Binary, varint::VarInt};
use crate::packet::{CompressedPacket, Packet, PacketData};
use self::{error::{NetworkError, Result}, options::HandlerOptions};

pub struct ClientNetworkHandler {
    tcp_stream: TcpStream,
    options: HandlerOptions,
}

impl ClientNetworkHandler {
    pub fn new(tcp_stream: TcpStream) -> Self {
        Self::with_options(tcp_stream, HandlerOptions::default())
    }

    pub fn with_options(tcp_stream: TcpStream, options: HandlerOptions) -> Self {
        Self { tcp_stream, options }
    }

    pub fn get_options_mut(&mut self) -> &mut HandlerOptions {
        &mut self.options
    }

    pub fn get_options(&mut self) -> &HandlerOptions {
        &self.options
    }

    pub async fn read_packet(&mut self) -> Result<Packet> {
        let length = read_varint::from_reader(&mut self.tcp_stream).await.map_err(|_| NetworkError::ProtocolError)?.0;
        if length < 0 {
            return Err(NetworkError::ConnectionClose);
        }
        let length = length as usize;
        let mut bin = vec![0_u8; length];
        self.tcp_stream.read_exact(&mut bin).await.map_err(|_| NetworkError::ProtocolError)?;
        if self.options.is_compression_enabled() {
            // read compressed packet
            let compressed = CompressedPacket::from_binary(bin).map_err(|_error| NetworkError::ProtocolError)?;
            compressed.decompress().map_err(|error| NetworkError::IOError(error))
        } else {
            // read packet
            Packet::from_binary(bin).map_err(|error| NetworkError::ParseError(error))
        }
    }

    pub async fn read<P: PacketData + Binary>(&mut self) -> Result<P> {
        let packet = self.read_packet().await?; 
        P::from_packet(packet).map_err(|err| NetworkError::PacketError(err))
    }

    pub async fn write_packet(&mut self, packet: Packet) -> Result<()> {
        let mut packet_binary = if self.options.is_compression_enabled() {
            packet.compress(
                    self.options.threshold().unwrap_or_default() as usize,
                    self.options.compression().unwrap_or_default()
                    )
                  .map_err(|error| NetworkError::IOError(error))?
                  .to_binary()
        } else {
            packet.to_binary()
        };
        let binary = {
            let mut v = Vec::with_capacity(packet_binary.len() + 5);
            v.append(&mut VarInt::new(packet_binary.len() as i32).to_binary());
            v.append(&mut packet_binary);
            v
        };
        self.tcp_stream.write(&binary).await.map_err(|error| NetworkError::IOError(error))?;
        Ok(())
    }

    pub async fn write<P: PacketData + Binary>(&mut self, packet: P) -> Result<()> {
        self.write_packet(packet.to_packet()).await
    }
}