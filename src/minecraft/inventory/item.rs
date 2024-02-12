use crate::{format::nbt::NBTTag, bin_fmt, tools::{utils::bin::{Binary, BinaryError}, maths::varint::{to_vari32, from_vari32, vari32_byte_length}}};

// const LABEL_ENCHANTMENT: &'static str = "StoredEnchantments";
// const LABEL_ENCHANTMENT_IDENTIFIER: &'static str = "id";
// const LABEL_ENCHANTMEL_LEVEL: &'static str = "lvl";

#[repr(i32)]
#[derive(Clone, Copy, Debug)]
pub enum Material {
    Air=0,
    Stone,
}

impl std::convert::TryFrom<i32> for Material {
    type Error = BinaryError;   // TODO : use other error 

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => {
                Ok(Material::Air)
            },
            1 => {
                Ok(Material::Stone)
            },
            _ => {
                Err(BinaryError::empty())
            }
        }
    }
}

impl Binary for Material {
    fn to_bin(&self) -> Vec<u8> {
        to_vari32(*self as i32)
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        if bin.len() == 0 {
            return Err(BinaryError::empty());
        }
        let material = Material::try_from(from_vari32(&bin[0..bin.len()]))?;
        Ok(material)
    }

    fn byte_length(&self) -> usize {
        vari32_byte_length(*self as i32)
    }
}

type OptionalNBT = Option<NBTTag>;
bin_fmt!(ItemStack => [material: Material, amount: i8, nbt: OptionalNBT]);


impl ItemStack {
    pub fn new(material: Material, amount: i8) -> Self {
        Self { material, amount, nbt: None }
    }

    // TODO : add MetaData
}