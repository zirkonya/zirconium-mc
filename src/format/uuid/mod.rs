/*
    0                   1                   2                   3
    0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                          time_low                             |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |       time_mid                |         time_hi_and_version   |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |clk_seq_hi_res |  clk_seq_low  |         node (0-1)            |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |                         node (2-5)                            |
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
*/

use core::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tools::utils::bin::Binary;

pub fn random<T>(seed: T, salt: T) -> T
    where T: Copy + std::ops::BitXorAssign + std::ops::Shr<u8, Output = T> + std::ops::Shl<u8, Output = T>
{
    let mut value = seed;
    value ^= salt;
    value ^= value << 21;
    value ^= value >> 35;
    value ^= value << 4;
    value
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Uuid {
    data: u128
}

impl Uuid {
    pub fn new() -> Self {
        const SALT: u128 = 0x8dc241e9d70a730f7f130b5cbf992eb6_u128;
        let seed = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error while getting date").as_millis();
        // TODO: RFC 4122
        let data = random(seed, SALT) | 0x00000000_0000_4000_a000_000000000000;
        Self { data }
    }

    pub fn nil() -> Self {
        Uuid { data: u128::MIN }
    }

    pub fn max() -> Self {
        Uuid { data: u128::MAX }
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, &byte) in self.data.to_le_bytes().iter().enumerate() {
            write!(f, "{:02x}", byte)?;
            if i == 3 || i == 5 || i == 7 || i == 9 {
                write!(f, "-")?;
            }
        }
        Ok(())
    }
}

impl Binary for Uuid {
    fn to_bin(&self) -> Vec<u8> {
        self.data.to_bin()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        Ok(Self { data: u128::from_bin(bin)? })
    }

    fn byte_length(&self) -> usize {
        16
    }
}