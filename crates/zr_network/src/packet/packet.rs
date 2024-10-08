use super::{compressed::CompressedPacket, PacketData};
use crate::error::packet::PacketError;
use flate2::Compression;
use std::fmt::Debug;
use std::io;
use zr_binary::{binary::Binary, varint::VarInt};

#[derive(Clone, Eq, PartialEq)]
pub struct Packet {
    pub(super) id: VarInt<i32>,
    pub(super) data: Vec<u8>,
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Packet {{ id: {:02x}, data: {} }}",
            self.id.0,
            self.data
                .iter()
                .map(|x| format!("{x:02x}"))
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl Packet {
    pub fn id(&self) -> i32 {
        self.id.0
    }

    pub fn data<P: PacketData + Binary>(self) -> Result<P, PacketError> {
        P::from_packet(self)
    }

    pub fn raw_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn compress(
        self,
        threshold: usize,
        compression: Compression,
    ) -> io::Result<CompressedPacket> {
        if self.binary_len() < threshold {
            Ok(CompressedPacket::uncompressed(self))
        } else {
            CompressedPacket::compressed(self, compression)
        }
    }
}

impl Binary for Packet {
    fn to_binary(mut self) -> Vec<u8> {
        let mut v = self.id.to_binary();
        v.append(&mut self.data);
        v
    }

    fn binary_len(&self) -> usize {
        self.id.binary_len() + self.data.binary_len()
    }

    fn from_binary(bin: Vec<u8>) -> zr_binary::error::Result<Self>
    where
        Self: Sized,
    {
        let id = VarInt::<i32>::from_binary(bin[0..].to_vec())?;
        let cursor = id.binary_len();
        let data = bin[cursor..].to_vec();
        Ok(Self { id, data })
    }
}
