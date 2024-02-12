use crate::{err, tools::{utils::bin::{Binary, BinaryError}, maths::varint::vari32_byte_length}};
// TODO : update
#[derive(Clone, Default)]
pub struct Chat(String);

err!(ChatError { message: String });

impl ChatError {
    pub fn too_long(current: usize) -> ChatError {
        Self { message: format!("Chat too long {} max: {}", current, 262_144_usize) }
    }
}

impl Chat {
    pub fn new(string: String) -> Result<Self, ChatError> {
        if string.len() <= 262_144_usize {
            Ok(Self(string))
        } else {
            Err(ChatError::too_long(string.len()))
        }
    }
}

impl Binary for Chat {
    fn to_bin(&self) -> Vec<u8> {
        self.0.to_bin()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, BinaryError> where Self: Sized {
        if bin.len() == 0 {
            return Err(BinaryError::empty());
        }
        let string = String::from_bin(bin)?;
        if string.len() > 262_144_usize {
            return Err(BinaryError::new(format!("Chat too long : {}", string.len())));
        }
        Ok(Chat(string))
    }

    fn byte_length(&self) -> usize {
        if self.0.len() == 0 {
            1
        } else {
            vari32_byte_length(self.0.len() as i32) + self.0.len() * 4
        }
    }
}