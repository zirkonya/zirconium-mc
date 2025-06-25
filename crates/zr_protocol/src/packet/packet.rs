use crate::{packet::PacketData, parser::binary::ToBytes, varint::VarInt};

pub struct Packet<D>
where
    D: PacketData + ToBytes,
{
    id: VarInt<i32>,
    payload: D,
}

impl<D> Packet<D>
where
    D: PacketData + ToBytes,
{
    pub(super) fn new(id: i32, payload: D) -> Self {
        Self {
            id: VarInt::new(id),
            payload,
        }
    }

    pub fn into_payload(self) -> D {
        self.payload
    }

    pub fn payload(&self) -> &D {
        &self.payload
    }
}

impl<D> ToBytes for Packet<D>
where
    D: PacketData + ToBytes,
{
    fn bytes_len(&self) -> usize {
        self.id.size() + self.payload.bytes_len()
    }

    fn to_bytes<B>(&self) -> Result<(usize, B), ()>
    where
        B: From<Vec<u8>>,
    {
        let (id_size, mut buffer): (_, Vec<u8>) = self.id.to_bytes()?;
        let (payload_size, mut payload): (_, Vec<u8>) = self.payload.to_bytes()?;
        buffer.append(&mut payload);
        Ok((id_size + payload_size, B::from(buffer)))
    }

    fn from_bytes<B>(bytes: B) -> Result<(usize, Self), ()>
    where
        B: Into<Vec<u8>>,
        Self: Sized,
    {
        let bytes: Vec<u8> = bytes.into();
        let (id_size, id) = VarInt::<i32>::from_bytes(&bytes[0..VarInt::<i32>::max_size()])?;
        let (payload_size, payload) = D::from_bytes(&bytes[id_size..])?;
        Ok((id_size + payload_size, Self { id, payload }))
    }
}
