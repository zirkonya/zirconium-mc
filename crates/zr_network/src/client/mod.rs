use crate::{
    error::{network::NetworkError, packet::PacketError},
    packet::packet::Packet,
};
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};
use zr_binary::{
    binary::Binary,
    error::BinaryError,
    varint::{self, VarInt},
};

pub mod manager;

const MAX_SIZE: usize = 2_097_149;

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Result<Self, Error> {
        stream.set_nonblocking(true)?;
        Ok(Self { stream })
    }

    pub fn shutdown(&mut self) -> Result<(), std::io::Error> {
        self.stream.shutdown(std::net::Shutdown::Both)
    }

    pub fn read_packet(&mut self) -> Result<Packet, NetworkError> {
        let len = varint::from_reader(&mut self.stream)
            .map_err(NetworkError::IOError)?
            .map_err(|err| NetworkError::PacketError(PacketError::DataError(err)))?;
        if (len.0 as usize) < MAX_SIZE {
            return Err(NetworkError::PacketError(PacketError::DataError(
                BinaryError::LengthError,
            )));
        }
        let mut buf = vec![0_u8; len.0 as usize];
        self.stream.read(&mut buf).map_err(NetworkError::IOError)?;
        Packet::from_binary(buf)
            .map_err(|err| NetworkError::PacketError(PacketError::DataError(err)))
    }

    pub fn write_packet(&mut self, packet: Packet) -> Result<(), NetworkError> {
        let len: VarInt<i32> = packet.binary_len().into();
        let mut vec = len.to_binary();
        vec.extend(packet.to_binary());
        self.stream.write(&vec).map_err(NetworkError::IOError)?;
        Ok(())
    }

    pub fn try_clone(&self) -> Self {
        Self {
            stream: self.stream.try_clone().expect("Cannot clone stream"), // TODO : handle error
        }
    }
}
