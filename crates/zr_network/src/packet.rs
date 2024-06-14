use std::io::{self, Write};
use flate2::write::ZlibEncoder;
use zr_binary::binary::Binary;
use zr_binary::varint::VarInt;

use crate::error::{self, PacketError};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Packet {
    id: VarInt<i32>,
    data: Vec<u8>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CompressedPacket {
    data_length: VarInt<i32>,
    compressed_packet: Vec<u8>,
}

pub trait PacketData {
    const ID: i32;
    fn id() -> i32 { Self::ID }
    
    fn to_packet(self) -> Packet where Self: Binary + Sized {
        Packet { id: Self::ID.into(), data: self.to_binary() }
    }
    
    fn from_compressed(packet: CompressedPacket) -> crate::error::Result<Self> where Self: Binary + Sized {
        let packet = packet.decompress().map_err(|error| error::PacketError::IOError(error))?;
        Ok(Self::from_packet(packet)?)
    }

    fn from_packet(packet: Packet) -> crate::error::Result<Self> where Self: Binary + Sized {
        assert_eq!(packet.id, Self::ID.into());
        if packet.id != Self::ID.into() {
            Err(PacketError::WrongId { expected: Self::ID, found: packet.id() })
        } else {
            Self::from_binary(packet.data).map_err(|error| error::PacketError::DataError(error))
        }
    }
}

impl Packet {
    pub fn new<Data: Binary + PacketData>(data: Data) -> Self {
        Self {
            id: Data::id().into(),
            data: data.to_binary()
        }
    }

    pub fn compress(self, threshold: usize, compression: flate2::Compression) -> io::Result<CompressedPacket> {
        if self.binary_len() <= threshold {
            Ok(CompressedPacket::uncompressed(self))
        } else {
            CompressedPacket::compressed(self, compression)
        }
    }

    pub fn id(&self) -> i32 {
        self.id.into()
    }

    pub fn data<P: PacketData + Binary>(self) -> Result<P, PacketError> {
        if self.id.0 != P::ID {
            return Err(PacketError::WrongId { expected: P::ID, found: self.id.0 })
        }
        Ok(P::from_binary(self.data).map_err(|error| PacketError::DataError(error))?)
    }

    pub fn raw_data(&self) -> &Vec<u8> {
        &self.data
    }
}

impl CompressedPacket {
    pub fn decompress(&self) -> io::Result<Packet> {
        todo!()
    }

    fn uncompressed(origin: Packet) -> Self {
        Self { 
            data_length: VarInt::default(),
            compressed_packet: origin.to_binary()
        }
    }

    fn compressed(origin: Packet, compression: flate2::Compression) -> io::Result<Self> {
        let data_length = origin.binary_len() as i32;
        assert!(data_length > 0);
        let compressed_packet = {
            let mut encoder = ZlibEncoder::new(Vec::new(), compression);
            encoder.write_all(&origin.to_binary())?;
            encoder.finish()?
        };
        Ok(Self { data_length: data_length.into(), compressed_packet })
    }
}

impl Binary for Packet {
    fn binary_len(&self) -> usize {
        self.id.binary_len() +
        self.data.len()
    }

    fn to_binary(mut self) -> Vec<u8> {
        let mut v = self.id.to_binary();
        v.append(&mut self.data);
        v
    }

    fn from_binary(bin: Vec<u8>) -> zr_binary::error::Result<Self> where Self: Sized {
        let id = VarInt::<i32>::from_binary(bin[..5.min(bin.len())].to_vec())?;
        let cursor = id.binary_len();
        let data = bin[cursor..].to_vec();
        Ok(Self { id, data })
    }
}

impl Binary for CompressedPacket {
    fn binary_len(&self) -> usize {
        self.data_length.binary_len() +
        self.compressed_packet.len()
    }

    fn to_binary(mut self) -> Vec<u8> {
        let mut v = self.data_length.to_binary();
        v.append(&mut self.compressed_packet);
        v
    }

    fn from_binary(bin: Vec<u8>) -> zr_binary::error::Result<Self> where Self: Sized {
        let data_length = VarInt::<i32>::from_binary(bin[..5].to_vec())?;
        let cursor = data_length.binary_len();
        let compressed_packet = bin[cursor..].to_vec();
        Ok(Self { data_length, compressed_packet })
    }
}