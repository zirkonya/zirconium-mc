use std::io::{self, Read};

use flate2::{
    bufread::{ZlibDecoder, ZlibEncoder},
    Compress, Compression,
};
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
        let date_length = origin.binary_len() as i32;
        let compressed_packet = {
            let mut binary: Vec<u8> = Vec::new();
            let bin_packet = origin.to_binary();
            let mut encoder = ZlibEncoder::new_with_compress(
                bin_packet.as_slice(),
                Compress::new(compression, true),
            );
            encoder.read_to_end(&mut binary)?;
            binary
        };
        Ok(Self {
            data_length: date_length.into(),
            compressed_packet,
        })
    }

    pub fn decompress(self) -> io::Result<Packet> {
        println!("GET COMPRESSED");
        Ok(if self.data_length.0 == 0_i32 {
            Packet::from_binary(self.compressed_packet).unwrap()
        } else {
            let mut binary: Vec<u8> = Vec::new();
            let mut decoder = ZlibDecoder::new(self.compressed_packet.as_slice());
            decoder.read_to_end(&mut binary)?;
            Packet::from_binary(binary).unwrap()
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
