use std::io;

use tokio::io::AsyncReadExt;
use zr_binary::varint::VarInt;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub async fn from_reader<R: AsyncReadExt + std::marker::Unpin>(
    reader: &mut R,
) -> io::Result<VarInt<i32>> {
    let mut value = 0;
    let mut position = 0;
    loop {
        let read_byte = reader.read_u8().await?;
        value |= ((read_byte & SEGMENT_BITS) as i32) << position;
        if read_byte & CONTINUE_BIT == 0 {
            break Ok(VarInt::new(value));
        }
        position += 7;
        if position >= 32 {
            break Err(io::Error::new(io::ErrorKind::Other, "FormatError"))
        }
    }
}