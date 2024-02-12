use crate::tools::{maths::varint::varint::VarInt, utils::{bin::{Binary, BinaryError}, compress::{compress, decompress}}};

pub trait IPacket {
    // TODO : send via TCP to client
    // TODO : read via TCP to sender    
}

pub struct Packet<D: Binary> {
    id: VarInt<i32>,
    data: D
}

pub struct CompressedPacket<D: Binary> {
    id: VarInt<i32>,
    data: D
}

impl<D: Binary> IPacket for Packet<D> {}
impl<D: Binary> IPacket for CompressedPacket<D> {}

impl<D: Binary> Packet<D> {
    pub fn new(id: i32, data: D) -> Self {
        Self {
            id: VarInt::new(id),
            data
        }
    }

    pub fn id(&self) -> i32 {
        self.id.value()
    }

    pub fn data(&self) -> &D {
        &self.data
    }
}

impl<D: Binary> CompressedPacket<D> {
    pub fn new(id: i32, data: D) -> Self {
        Self {
            id: VarInt::new(id),
            data
        }
    }

    pub fn id(&self) -> i32 {
        self.id.value()
    }

    pub fn data(&self) -> &D {
        &self.data
    }
}

impl <D: Binary> Binary for Packet<D> {
    fn to_bin(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        vec.append(&mut self.id.to_bin());
        vec.append(&mut VarInt::new(self.byte_length() as i32).to_bin());
        vec.append(&mut self.data.to_bin());
        vec
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let id = VarInt::from_bin(bin.clone())?;
        let mut cursor = id.byte_length();
        let size: VarInt<i32> = VarInt::from_bin(bin[cursor..].to_vec())?;
        cursor += size.byte_length();
        if bin.len() < cursor + size.value() as usize {
            return Err(BinaryError::wrong_size(size.value() as usize, bin.len() - cursor));
        }

        let data = D::from_bin(bin[cursor..cursor + size.value() as usize].to_vec())?;

        Ok(Self {
            id,
            data
        })
    }

    fn byte_length(&self) -> usize {
        let size = self.id.byte_length() + self.data.byte_length();
        size + VarInt::new(size as i32).byte_length()
    }
}

impl <D: Binary> Binary for CompressedPacket<D> {
    fn to_bin(&self) -> Vec<u8> {
        let mut compressed: Vec<u8>;
        let data_len: VarInt<i32>;

        {
            let mut to_compress = self.id.to_bin();
            to_compress.append(&mut self.data.to_bin());
            data_len = VarInt::new(to_compress.len() as i32);
            compressed = compress(&to_compress).unwrap_or_default();
        }

        let packet_len = VarInt::new((compressed.len() + data_len.byte_length() ) as i32);
        let mut vec = packet_len.to_bin();
        vec.append(&mut data_len.to_bin());
        vec.append(&mut compressed);
        
        vec
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        let packet_len = VarInt::<i32>::from_bin(bin.clone())?;
        let mut cursor = packet_len.byte_length();

        if bin.len() < cursor + packet_len.value() as usize {
            return Err(BinaryError::wrong_size(packet_len.value() as usize, cursor));
        }

        let data_len = VarInt::<i32>::from_bin(bin[cursor..].to_vec())?;
        cursor += data_len.byte_length();
        let packet = decompress(&bin[cursor..(data_len.value() as usize + cursor)]).unwrap_or_default();
        let id = VarInt::<i32>::from_bin(packet.clone())?;
        let data = D::from_bin(packet[id.byte_length()..].to_vec())?;
        
        Ok(Self {
            id,
            data
        })
    }

    fn byte_length(&self) -> usize {
        // size of (packet len) + size of (data len) + size of compressed (id + data)
        let data_len = self.data.byte_length() + self.id.byte_length();
        let compressed_size: usize;
        
        {

            let mut to_compress = self.id.to_bin();
            to_compress.append(&mut self.data.to_bin());
            let compressed = compress(&to_compress).unwrap_or_default();
            compressed_size = compressed.len();
        }

        data_len + VarInt::new(data_len as i32).byte_length() + compressed_size
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_packet() {

    }

    #[test]
    fn test_compressed_packet() {

    }
}