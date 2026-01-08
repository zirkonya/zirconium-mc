use crate::{
    network::context::{Bound, ClientContext, State},
    utils::varint::VarInt,
};
use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::{fmt::Debug, sync::Arc};
use zr_protocol::{
    macros::Serialize,
    message::size::PrefixSize,
    serialization::{Serialize, SerializeWithContext},
};

#[derive(Debug, Clone)]
pub struct Bundle {
    pub packets: Arc<[PrefixSize<VarInt<i32>, Arc<[u8]>>]>,
}

impl Serialize for Bundle {
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> std::io::Result<()> {
        let packets = self.packets.iter();
        for packet in packets {
            packet.write_to(buffer)?;
        }
        Ok(())
    }

    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let mut buffer = Vec::with_capacity(2048);
        loop {
            if let Ok(packet) = PrefixSize::read_from(reader) {
                buffer.push(packet);
            } else {
                break;
            }
        }
        buffer.shrink_to_fit(); // really usefull ?
        Ok(Self {
            packets: buffer.into(),
        })
    }
}

impl Bundle {
    pub fn into_packets(self, ctx: &ClientContext) -> Vec<std::io::Result<Packet>> {
        self.packets
            .iter()
            .map(|bytes| Packet::from_bytes_with_context(bytes.data(), ctx))
            .collect::<Vec<_>>()
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Packet {
    pub(crate) id: VarInt<i32>,
    #[skip]
    pub bound: Option<Bound>,
    #[skip]
    pub state: Option<State>,
    pub(crate) payload: Arc<[u8]>,
}

impl Packet {
    pub fn new(id: i32, payload: Arc<[u8]>) -> Self {
        Self {
            id: VarInt::new(id),
            bound: None,
            state: None,
            payload,
        }
    }

    pub fn id(&self) -> &i32 {
        self.id.value()
    }

    pub fn bound(&self) -> &Option<Bound> {
        &self.bound
    }

    pub fn state(&self) -> &Option<State> {
        &self.state
    }

    pub fn payload(&self) -> Arc<[u8]> {
        self.payload.clone()
    }
}

#[derive(Clone, Debug)]
pub struct WithCompression(Packet);

impl From<WithCompression> for Packet {
    fn from(val: WithCompression) -> Self {
        val.0
    }
}

impl From<Packet> for WithCompression {
    fn from(packet: Packet) -> Self {
        Self(packet)
    }
}

impl SerializeWithContext for WithCompression {
    type Context = ClientContext;

    fn write_to_with_context<W: std::io::Write>(
        &self,
        buffer: &mut W,
        ctx: &Self::Context,
    ) -> std::io::Result<()> {
        let packet = self.0.to_bytes()?;
        let threshold = ctx.compression_threshold.unwrap_or(0) as usize;

        if packet.len() < threshold {
            VarInt::new(0).write_to(buffer)?;
            buffer.write_all(&packet)?;
        } else {
            use std::io::prelude::*;
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&packet)?;
            let compressed = encoder.finish()?;

            VarInt::new(packet.len() as i32).write_to(buffer)?;
            buffer.write_all(&compressed)?;
        }
        Ok(())
    }

    fn read_from_with_context<R: std::io::Read>(
        reader: &mut R,
        _: &Self::Context,
    ) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let data_len = VarInt::<i32>::read_from(reader)?;
        if *data_len.value() == 0 {
            Ok(WithCompression(Packet::read_from(reader)?))
        } else {
            use std::io::prelude::*;
            let mut z = ZlibDecoder::new(reader);
            let mut data = Vec::new();
            z.read_to_end(&mut data)?;
            Ok(WithCompression(Packet::from_bytes(&data)?))
        }
    }
}

impl SerializeWithContext for Packet {
    type Context = ClientContext;

    fn write_to_with_context<W: std::io::Write>(
        &self,
        buffer: &mut W,
        ctx: &Self::Context,
    ) -> std::io::Result<()> {
        if ctx.compression_set {
            WithCompression(self.clone()).write_to_with_context(buffer, ctx)?;
        } else {
            self.write_to(buffer)?;
        }
        Ok(())
    }

    fn read_from_with_context<R: std::io::Read>(
        reader: &mut R,
        ctx: &Self::Context,
    ) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let mut packet = if ctx.compression_set {
            WithCompression::read_from_with_context(reader, ctx)?.into()
        } else {
            Self::read_from(reader)?
        };
        packet.bound = Some(Bound::Server); // TODO : contextualize
        packet.state = Some(ctx.state);
        Ok(packet)
    }
}
