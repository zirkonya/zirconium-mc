use std::io::{self, Write};

use flate2::{write::ZlibEncoder, Compression};
use zr_binary::{binary::Binary, varint::VarInt};

use super::packet::Packet;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompressedPacket {
    pub(super) data_length: VarInt<i32>,
    pub(super) compressed_packet: Vec<u8>,
}

impl CompressedPacket {
    pub fn uncompressed(origin: Packet) -> Self {
        Self {
            data_length: VarInt::default(),
            compressed_packet: origin.to_binary(),
        }
    }

    pub fn compressed(origin: Packet, compression: Compression) -> io::Result<Self> {
        let date_length = origin.binary_len() as i32; // TODO : verify is size is valid
        let compressed_packet = {
            let mut encoder = ZlibEncoder::new(Vec::new(), compression);
            encoder.write_all(&origin.to_binary())?;
            encoder.finish()?
        };
        Ok(Self {
            data_length: date_length.into(),
            compressed_packet,
        })
    }
}

impl Binary for CompressedPacket {
    fn from_binary(bin: Vec<u8>) -> zr_binary::error::Result<Self>
    where
        Self: Sized,
    {
        let data_length = VarInt::<i32>::from_binary(bin[0..5].to_vec())?;
        let cursor = data_length.binary_len();
        let compressed_packet = bin[cursor..].to_vec();
        Ok(Self {
            data_length,
            compressed_packet,
        })
    }

    fn binary_len(&self) -> usize {
        self.data_length.binary_len() + self.compressed_packet.binary_len()
    }

    fn to_binary(mut self) -> Vec<u8> {
        let mut v = self.data_length.to_binary();
        v.append(&mut self.compressed_packet);
        v
    }
}
