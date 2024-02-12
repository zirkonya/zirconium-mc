use std::any::Any;

use crate::{err, tools::utils::{bin::{Binary, BinaryError}, optional::Optional}};
pub type OptionalString = Optional<String>;

err!(NBTError { tag_id: u8, message: String });

#[repr(u8)]
#[derive(Clone, PartialEq, Debug)]
pub enum NBTTag {
    End = 0,
    Byte { label: OptionalString, value: i8 },
    Short { label: OptionalString, value: i16 },
    Int { label: OptionalString, value: i32 },
    Long { label: OptionalString, value: i64 },
    Float { label: OptionalString, value: f32 },
    Double { label: OptionalString, value: f64 },
    ByteArray { label: OptionalString, value: Vec<u8> },
    String { label: OptionalString, value: String },
    List { label: OptionalString, value: Vec<Self> },
    Compound { label: OptionalString, value: Vec<Self> },
    IntArray { label: OptionalString, value: Vec<i32> },
    LongArray { label: OptionalString, value: Vec<i64> }
}

impl Default for NBTTag {
    fn default() -> Self {
        Self::Compound { label: Optional::None, value: Vec::new() }
    }
}

impl NBTTag {
    pub fn id(&self) -> u8 {
        match self {
            NBTTag::End => 0x00,
            NBTTag::Byte { label: _, value: _ } => 0x01,
            NBTTag::Short { label: _, value: _ } => 0x02,
            NBTTag::Int { label: _, value: _ } => 0x03,
            NBTTag::Long { label: _, value: _ } => 0x04,
            NBTTag::Float { label: _, value: _ } => 0x05,
            NBTTag::Double { label: _, value: _ } => 0x06,
            NBTTag::ByteArray { label: _, value: _ } => 0x07,
            NBTTag::String { label: _, value: _ } => 0x08,
            NBTTag::List { label: _, value: _ } => 0x09,
            NBTTag::Compound { label: _, value: _ } => 0x0A,
            NBTTag::IntArray { label: _, value: _ } => 0x0B,
            NBTTag::LongArray { label: _, value: _ } => 0x0C,
        }
    }

    pub fn label(&self) -> OptionalString {
        match self {
            NBTTag::End => Optional::None,
            NBTTag::Byte { label, value: _ } => label.clone(),
            NBTTag::Short { label, value: _ } => label.clone(),
            NBTTag::Int { label, value: _ } => label.clone(),
            NBTTag::Long { label, value: _ } => label.clone(),
            NBTTag::Float { label, value: _ } => label.clone(),
            NBTTag::Double { label, value: _ } => label.clone(),
            NBTTag::ByteArray { label, value: _ } => label.clone(),
            NBTTag::String { label, value: _ } => label.clone(),
            NBTTag::List { label, value: _ } => label.clone(),
            NBTTag::Compound { label, value: _ } => label.clone(),
            NBTTag::IntArray { label, value: _ } => label.clone(),
            NBTTag::LongArray { label, value: _ } => label.clone(),
        }
    }

    pub fn bin_value(&self) -> Vec<u8> {
        match self {
            NBTTag::End => vec![],
            NBTTag::Byte { label: _, value } => (*value).to_bin(),
            NBTTag::Short { label: _, value } => (*value).to_bin(),
            NBTTag::Int { label: _, value } => (*value).to_bin(),
            NBTTag::Long { label: _, value } => (*value).to_bin(),
            NBTTag::Float { label: _, value } => (*value).to_bin(),
            NBTTag::Double { label: _, value } => (*value).to_bin(),
            NBTTag::ByteArray { label: _, value } => value.clone(),
            NBTTag::String { label: _, value } => (*value).to_bin(),
            NBTTag::List { label: _, value } => (*value).to_bin(),
            NBTTag::Compound { label: _, value } => (*value).to_bin(),
            NBTTag::IntArray { label: _, value } => (*value).to_bin(),
            NBTTag::LongArray { label: _, value } => (*value).to_bin(),
        }
    }

    pub fn from_unnamed_bin(bin: Vec<u8>) -> Result<Self, BinaryError> {
        let tag_type = bin[0];
        let mut cursor = 1;
        let tag = match tag_type {
            0x00 => {
                Self::End
            },
            0x01 => {
                let label = Optional::None;
                let value = i8::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Byte { label, value }
            },
            0x02 => {
                let label = Optional::None;
                let value = i16::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Short { label, value }
            },
            0x03 => {
                let label = Optional::None;
                let value = i32::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Int { label, value }
            },
            0x04 => {
                let label = Optional::None;
                let value = i64::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Long { label, value }
            },
            0x05 => {
                let label = Optional::None;
                let value = f32::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Float { label, value }
            },
            0x06 => {
                let label = Optional::None;
                let value = f64::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Double { label, value }
            },
            0x07 => {
                let label = Optional::None;
                let value = Vec::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::ByteArray { label, value }
            },
            0x08 => {
                let label = Optional::None;
                let value = String::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::String { label, value }
            },
            0x09 => {
                let label = Optional::None;
                let len = i32::from_bin(bin[cursor..=(cursor+4)].to_vec())?;
                cursor += len.byte_length();
                let mut value = Vec::new();
                let mut list_tag_type = Optional::None;
                for _ in 0..len {
                    let tag = NBTTag::from_unnamed_bin(bin[cursor..bin.len()].to_vec())?;
                    if list_tag_type.is_none() {
                        value.push(tag.clone());
                        list_tag_type = Optional::Some(tag.id());
                        cursor += tag.byte_length();
                    } else if list_tag_type.unwrap() == tag.id() {
                        value.push(tag.clone());
                        cursor += tag.byte_length();
                    } else {
                        return Err(BinaryError::new("Wrong tag type in list".to_string()));
                    }
                }
                Self::List { label, value }            },
            0x0a => {
                let mut value = Vec::new();
                loop {
                    if cursor >= bin.len() {
                        return Err(BinaryError::new("No end tag".to_string()));
                    }
                    let tag = NBTTag::from_bin(bin[cursor..bin.len()].to_vec())?;
                    if tag.id() == 0x00 {
                        break;
                    }
                    value.push(tag.clone());
                    cursor += tag.byte_length();
                }
                Self::Compound { label: Optional::None, value }
            },
            0x0b => {
                let label = Optional::None;
                let value = Vec::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::IntArray { label, value }
            },
            0x0c => { 
                let label = Optional::None;
                let value = Vec::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::LongArray { label, value }
            },
            _ => {
                return Err(BinaryError::new(format!("Wrong id {:02x}", tag_type)));
            }
        };
        Ok(tag)
    }

    pub fn get_child(&self, name: String) -> Result<Self, NBTError> {
        match self {
            NBTTag::Compound { label: _, value } => {
                let opt = value.iter().find(|&elem| elem.label().unwrap_or_default() == name);
                match opt {
                    Some(child) => Ok(child.clone()),
                    None => Err(NBTError { tag_id: 255, message: "Tag not found".to_string() })
                }
            },
            _ => Err(NBTError { tag_id: self.id(), message: format!("Unvailable for {}", self.to_string()) })
        }
    }

    pub fn get_mut_child(&mut self, name: String) -> Result<&mut Self, NBTError> {
        match self {
            NBTTag::Compound { label: _, value } => {
                let mut opt = Optional::None; 
                for elem in value {
                    if elem.label().is_some() && elem.label().unwrap() == name {
                        opt = Optional::Some(elem);
                    }
                }
                match opt {
                    Optional::Some(child) => Ok(child),
                    Optional::None => Err(NBTError { tag_id: 255, message: "Tag not found".to_string() })
                }
            },
            _ => Err(NBTError { tag_id: self.id(), message: format!("Unvailable for {}", self.to_string()) })
        }
    }

    pub fn mut_value(&mut self) -> &mut dyn Any {
        match self {
            NBTTag::End => unimplemented!("no value in end tag"),
            NBTTag::Byte { label: _, value } => value,
            NBTTag::Short { label: _, value } => value,
            NBTTag::Int { label: _, value } => value,
            NBTTag::Long { label: _, value } => value,
            NBTTag::Float { label: _, value } => value,
            NBTTag::Double { label: _, value } => value,
            NBTTag::ByteArray { label: _, value } => value,
            NBTTag::String { label: _, value } => value,
            NBTTag::List { label: _, value } => value,
            NBTTag::Compound { label: _, value } => value,
            NBTTag::IntArray { label: _, value } => value,
            NBTTag::LongArray { label: _, value } => value,
        }
    }
}

impl ToString for NBTTag {
    fn to_string(&self) -> String {
        match self {
            NBTTag::End => String::from("End"),
            NBTTag::Byte { label, value } => format!("Byte({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::Short { label, value } => format!("Short({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::Int { label, value } => format!("Int({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::Long { label, value } => format!("Long({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::Float { label, value } => format!("Float({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::Double { label, value } => format!("Double({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::ByteArray { label, value } => format!("ByteArray({}) - {} ", label.clone().unwrap_or("---".to_string()), value.iter().map(|elem| format!("{:02x}", elem)).collect::<String>()),
            NBTTag::String { label, value } => format!("String({}) - {} ", label.clone().unwrap_or("---".to_string()), value),
            NBTTag::List { label, value } => format!("List({}) - {} ", label.clone().unwrap_or("---".to_string()), value.iter().map(|tag| tag.to_string()).collect::<String>()),
            NBTTag::Compound { label, value } => format!("Compound({}) - {} ", label.clone().unwrap_or("---".to_string()), value.iter().map(|tag| tag.to_string()).collect::<String>()),
            NBTTag::IntArray { label, value } => format!("IntArray({}) - {} ", label.clone().unwrap_or("---".to_string()), value.iter().map(|int| int.to_string()).collect::<Vec<String>>().join(", ")),
            NBTTag::LongArray { label, value } => format!("LongArray({}) - {} ", label.clone().unwrap_or("---".to_string()), value.iter().map(|long| long.to_string()).collect::<Vec<String>>().join(", ")),
        }
    }
}

impl Binary for OptionalString {
    fn to_bin(&self) -> Vec<u8> {
        if self.is_none() {
            Vec::new()
        } else {
            let mut label = self.clone().unwrap().as_bytes().to_vec();
            let mut bin = (label.len() as i32).to_bin();
            bin.append(&mut label);
            bin
        }
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        if bin.len() == 0 {
            Ok(Optional::None)
        } else {
            let mut cursor = 0;
            let len = i32::from_bin(bin[cursor..=3].to_vec())?;
            cursor += len.byte_length();
            if len < 0 { Err(BinaryError::new("Weird string with negative size".to_string())) }
            else {
                if len as usize + cursor >= bin.len() { Err(BinaryError::wrong_size(len as usize + cursor, bin.len())) }
                else { Ok(Optional::Some(String::from_utf8_lossy(&bin[cursor..(len as usize + cursor)]).to_string())) }
            }
        }
    }

    fn byte_length(&self) -> usize {
        if self.is_none() {
            0
        } else {
            4 + self.clone().unwrap().len()
        }
    }
}

impl Binary for NBTTag {
    fn to_bin(&self) -> Vec<u8> {
        let mut bin = vec![self.id()];
        match self {
            NBTTag::Compound { label, value } => {
                bin.append(&mut label.to_bin());
                for elem in value {
                    bin.append(&mut elem.to_bin());
                }
                bin.push(0);
            },
            _ => {
                bin.append(&mut self.label().to_bin());
                bin.append(&mut self.bin_value());
            }
        }
        bin
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        let tag_type = bin[0];
        let mut cursor = 1;
        let tag = match tag_type {
            0x00 => {
                Self::End
            },
            0x01 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = i8::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Byte { label, value }
            },
            0x02 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = i16::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Short { label, value }
            },
            0x03 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = i32::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Int { label, value }
            },
            0x04 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = i64::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Long { label, value }
            },
            0x05 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = f32::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Float { label, value }
            },
            0x06 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = f64::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::Double { label, value }
            },
            0x07 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let len = i32::from_bin(bin[cursor..(cursor+4)].to_vec())?;
                cursor += len.byte_length();
                let value = Vec::from_bin(bin[cursor..(cursor + len as usize)].to_vec())?;
                Self::ByteArray { label, value }
            },
            0x08 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let value = String::from_bin(bin[cursor..bin.len()].to_vec())?;
                Self::String { label, value }
            },
            0x09 => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let len = i32::from_bin(bin[cursor..=(cursor+4)].to_vec())?;
                cursor += len.byte_length();
                let mut value = Vec::new();
                let mut list_tag_type = Optional::None;
                for _ in 0..len {
                    let tag = NBTTag::from_unnamed_bin(bin[cursor..bin.len()].to_vec())?;
                    if list_tag_type.is_none() {
                        list_tag_type = Optional::Some(tag.id());
                    }
                    if list_tag_type.unwrap() == tag.id() {
                        value.push(tag.clone());
                    } else {
                        return Err(BinaryError::new(format!("Wrong tag type in list; obtained: {:02x}; waited: {:02x};", tag.id(), list_tag_type.unwrap_or(0xFF))));
                    }
                    cursor += tag.byte_length();
                }
                Self::List { label, value }
            },
            0x0a => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let mut value = Vec::new();
                loop {
                    if cursor >= bin.len() {
                        return Err(BinaryError::new("No end tag".to_string()));
                    }
                    let tag = NBTTag::from_bin(bin[cursor..bin.len()].to_vec())?;
                    if tag.id() == 0x00 {
                        break;
                    }
                    value.push(tag.clone());
                    cursor += tag.byte_length();
                }
                Self::Compound { label, value }
            },
            0x0b => {
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let len = i32::from_bin(bin[cursor..(cursor+4)].to_vec())?;
                cursor += len.byte_length();
                let value = Vec::from_bin(bin[cursor..(cursor + (len as usize) * 4)].to_vec())?;
                Self::IntArray { label, value }
            },
            0x0c => { 
                let label = OptionalString::from_bin(bin[cursor..bin.len()].to_vec())?;
                cursor += label.byte_length();
                let len = i32::from_bin(bin[cursor..(cursor+4)].to_vec())?;
                cursor += len.byte_length();
                let value = Vec::from_bin(bin[cursor..(cursor + (len as usize) * 8)].to_vec())?;
                Self::LongArray { label, value }
            },
            _ => {
                return Err(BinaryError::new(format!("Wrong id 0x{:02x}", tag_type)));
            }
        };
        Ok(tag)
    }

    fn byte_length(&self) -> usize {
        1 + match self {
            NBTTag::End => 0,
            NBTTag::Byte { label, value: _ } => label.byte_length() + 1,
            NBTTag::Short { label, value: _ } => label.byte_length() + 2,
            NBTTag::Int { label, value: _ } => label.byte_length() + 4,
            NBTTag::Long { label, value: _ } => label.byte_length() + 8,
            NBTTag::Float { label, value: _ } => label.byte_length() + 4,
            NBTTag::Double { label, value: _ } => label.byte_length() + 8,
            NBTTag::ByteArray { label, value } => label.byte_length() + value.len(),
            NBTTag::String { label, value } => label.byte_length() + value.byte_length(),
            NBTTag::List { label, value } => label.byte_length() + value.byte_length(),
            NBTTag::Compound { label, value } => 1 + label.byte_length() + value.iter().map(|tag| tag.byte_length()).sum::<usize>(),
            NBTTag::IntArray { label, value } => label.byte_length() + value.byte_length(),
            NBTTag::LongArray { label, value } => label.byte_length() + value.byte_length(),
        }
    }
}

#[cfg(test)]
mod test {
    // TODO : test
}