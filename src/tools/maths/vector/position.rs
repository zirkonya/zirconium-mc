use crate::tools::utils::bin::Binary;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Position {
    x: i32,
    z: i32,
    y: i16
}

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, z, y  }
    }
}

impl Binary for Position {
    fn to_bin(&self) -> Vec<u8> {
        let x = self.x as u64;
        let z = self.z as u64;
        let y = self.y as u64;

        let long = ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF);
        long.to_bin()
    }

    fn from_bin(bin: Vec<u8>) -> Result<Self, crate::tools::utils::bin::BinaryError> where Self: Sized {
        let long = u64::from_bin(bin)?;
        let mut x = (long       >> 38) as i64;
        let mut y = (long << 52 >> 52) as i64;
        let mut z = (long << 26 >> 38) as i64;

        if x >= 1 << 25 { x -= 1 << 26; } 
        if y >= 1 << 11 { y -= 1 << 12; } 
        if z >= 1 << 25 { z -= 1 << 26; } 
        
        Ok(Position::new(x as i32, y as i16, z as i32))
    }

    fn byte_length(&self) -> usize {
        8
    }
}

#[cfg(test)]
mod tests {
    use crate::tools::utils::bin::Binary;

    use super::Position;

    #[test]
    fn test_positive_position() {
        let pos = Position::new(100, 64, 100);
        let bin = pos.to_bin();
        assert_eq!(pos, Position::from_bin(bin).expect("Error while converting"));
    }


    #[test]
    fn test_negative_position() {
        let pos = Position::new(-100, -64, -100);
        let bin = pos.to_bin();
        assert_eq!(pos, Position::from_bin(bin).expect("Error while converting"));
    }
}