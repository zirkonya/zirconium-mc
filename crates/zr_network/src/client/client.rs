use crate::{
    error::{network::NetworkError, packet::PacketError},
    packet::compressed::CompressedPacket,
    packet::packet::Packet,
};
use flate2::Compression;
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};
use zr_binary::{
    binary::Binary,
    error::BinaryError,
    varint::{self, VarInt},
};

const MAX_SIZE: usize = 2_097_149;

const CIPHER_ENABLE: u8 = 0b0000_0001;
pub const CIPHER_ACTIVE: u8 = 0b0000_0010;
const COMPRESSION_ENABLE: u8 = 0b0000_0100;
pub const COMPRESSION_ACTIVE: u8 = 0b0000_1000;

// TODO : Add cipher & compression
#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
    aes_key: [u8; 16],
    threshold: usize,
    compression: Compression,
    opt: u8,
}

impl Client {
    pub fn new(stream: TcpStream) -> Result<Self, Error> {
        Self::new_with_opt(stream, 0)
    }

    pub fn new_with_opt(stream: TcpStream, opt: u8) -> Result<Self, Error> {
        stream.set_nonblocking(true)?;
        let mut aes_key = [0_u8; 16];
        for i in 0..16 {
            aes_key[i] = rand::random();
        }
        Ok(Self {
            stream,
            threshold: MAX_SIZE,
            compression: flate2::Compression::default(),
            aes_key,
            opt,
        })
    }

    fn aes_key(&self) -> &[u8; 16] {
        &self.aes_key
    }

    fn is_cipher_enable(&self) -> bool {
        self.opt & CIPHER_ENABLE != 0
    }

    fn is_cipher_active(&self) -> bool {
        self.opt & CIPHER_ACTIVE != 0
    }

    fn is_compression_enable(&self) -> bool {
        self.opt & COMPRESSION_ENABLE != 0
    }

    fn is_compression_active(&self) -> bool {
        self.opt & COMPRESSION_ACTIVE != 0
    }

    pub fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    pub fn read_packet(&mut self) -> Result<Packet, NetworkError> {
        let len = varint::from_reader(&mut self.stream)
            .map_err(NetworkError::IOError)?
            .map_err(|err| NetworkError::PacketError(PacketError::DataError(err)))?;
        if (len.0 as usize) >= MAX_SIZE || len.0 < 0 {
            return Err(NetworkError::PacketError(PacketError::DataError(
                BinaryError::LengthError,
            )));
        }
        let mut buf = vec![0_u8; len.0 as usize];
        self.stream.read(&mut buf).map_err(NetworkError::IOError)?;
        let data = if self.is_cipher_enable() {
            let cipher = libaes::Cipher::new_128(&self.aes_key);
            cipher.cbc_decrypt(&[0; 16], &buf).to_vec()
        } else {
            buf.to_vec()
        };
        let packet = if self.is_compression_enable() {
            let compressed = CompressedPacket::from_binary(data)
                .map_err(|err| NetworkError::PacketError(PacketError::DataError(err)))?;
            compressed.decompress().map_err(NetworkError::IOError)
        } else {
            Packet::from_binary(data)
                .map_err(|err| NetworkError::PacketError(PacketError::DataError(err)))
        };
        println!("[R]\t{packet:?}");
        packet
    }

    pub fn write_packet(&mut self, packet: Packet) -> Result<(), NetworkError> {
        println!("[S]\t{packet:?}");
        let packet = if self.is_compression_enable() {
            packet
                .compress(self.threshold, self.compression)
                .map_err(NetworkError::IOError)?
                .to_binary()
        } else {
            packet.to_binary()
        };
        let data = {
            let len: VarInt<i32> = packet.len().into();
            let mut v = len.to_binary();
            v.extend(packet);
            v
        };
        let data = if self.is_cipher_enable() {
            let cipher = libaes::Cipher::new_128(&self.aes_key);
            cipher.cbc_encrypt(&[0; 16], &data)
        } else {
            data
        };
        self.stream.write(&data).map_err(NetworkError::IOError)?;
        Ok(())
    }

    pub fn active_compression(&mut self) {
        if self.is_compression_active() {
            self.opt |= COMPRESSION_ENABLE;
        }
    }

    pub fn active_cipher(&mut self) {
        if self.is_cipher_active() {
            self.opt |= COMPRESSION_ENABLE;
        }
    }

    pub fn try_clone(&self) -> std::io::Result<Self> {
        Ok(Self {
            stream: self.stream.try_clone()?,
            aes_key: self.aes_key.clone(),
            compression: self.compression.clone(),
            threshold: self.threshold,
            opt: self.opt,
        })
    }
}
