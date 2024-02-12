pub mod varint;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

/// The function `vari32_byte_length` calculates the number of bytes required to represent a given `i32`
/// value as a variable-length integer in Rust.
/// 
/// Arguments:
/// 
/// * `value`: The `value` parameter is of type `i32`, which represents a signed 32-bit integer.
/// 
/// Returns:
/// 
/// The function `vari32_byte_length` returns the number of bytes required to represent the given `i32`
/// value as a variable-length integer.
pub fn vari32_byte_length(value: i32) -> usize {
    let value = value as u32;
    if value < 128 {
        1
    } else if value < 16_384 {
        2
    } else if value < 2_097_152 {
        3
    } else if value < 268_435_456 {
        4
    } else {
        5
    }
}

/// The function `vari64_byte_length` calculates the number of bytes required to represent a given `i64`
/// value as a variable-length integer.
/// 
/// Arguments:
/// 
/// * `value`: The `value` parameter in the `vari64_byte_length` function is of type `i64`, which
/// represents a signed 64-bit integer.
/// 
/// Returns:
/// 
/// The function `vari64_byte_length` returns the number of bytes required to represent the given `i64`
/// value as a variable-length integer.
pub fn vari64_byte_length(value: i64) -> usize {
    let value = value as u64;
    if value < 128 {
        1
    } else if value < 16_384 {
        2
    } else if value < 2_097_152 {
        3
    } else if value < 268_435_456 {
        4
    } else if value < 34_359_738_368 {
        5
    } else if value < 4_398_046_511_104 {
        6
    } else if value < 562_949_953_421_312 {
        7
    } else if value < 72_057_594_037_927_936 {
        8
    } else {
        9
    }
}

/// The function "to_vari32" converts a signed 32-bit integer into a variable-length encoded vector of
/// bytes.
/// 
/// Arguments:
/// 
/// * `value`: The `value` parameter is of type `i32` and represents the integer value that needs to be
/// converted to a variable-length encoded format.
/// 
/// Returns:
/// 
/// The function `to_vari32` returns a `Vec<u8>`, which is a vector of unsigned 8-bit integers.
pub fn to_vari32(value: i32) -> Vec<u8> {
    let mut value = value;
    let mut vec = Vec::new();
    let len = vari32_byte_length(value);
    for _ in 1..len {
        vec.push(((value as u8) & SEGMENT_BITS) | CONTINUE_BIT);
        value >>= 7;
    }
    vec.push(value as u8 & SEGMENT_BITS);
    vec
}

/// The function `from_vari32` converts a byte array into a signed 32-bit integer using a
/// variable-length encoding scheme.
/// 
/// Arguments:
/// 
/// * `bytes`: A slice of u8 bytes that represents a variable-length encoded i32 value.
/// 
/// Returns:
/// 
/// an `i32` value.
pub fn from_vari32(bytes: &[u8]) -> i32 {
    let mut value: i32    = 0;
    let mut cursor: usize = 0;
    let mut position: i32 = 0;

    loop {
        let current_byte: u8 = bytes[cursor];
        value |= ((current_byte & SEGMENT_BITS) as i32) << position;
        if current_byte & CONTINUE_BIT == 0 { return value; }
        cursor += 1; position += 7;
        if position >= 32 { panic!("VarInt is too big") }
    }
}

/// The function "to_vari64" converts a signed 64-bit integer into a variable-length encoded byte array.
/// 
/// Arguments:
/// 
/// * `value`: The `value` parameter is of type `i64`, which means it is a signed 64-bit integer.
/// 
/// Returns:
/// 
/// The function `to_vari64` returns a `Vec<u8>`, which is a vector of unsigned 8-bit integers.
pub fn to_vari64(value: i64) -> Vec<u8> {
    let mut value = value;
    let mut vec = Vec::new();
    let len = vari64_byte_length(value);
    for _ in 1..len {
        vec.push(((value as u8) & SEGMENT_BITS) | CONTINUE_BIT);
        value >>= 7;
    }
    vec.push(value as u8 & SEGMENT_BITS);
    vec
}

/// The function `from_vari64` converts a byte array into a signed 64-bit integer using a
/// variable-length encoding scheme.
/// 
/// Arguments:
/// 
/// * `bytes`: A slice of u8 bytes that represents a variable-length integer.
pub fn from_vari64(bytes: &[u8]) -> i64 {
    let mut value: i64    = 0;
    let mut cursor: usize = 0;
    let mut position: i64 = 0;

    loop {
        let current_byte: u8 = bytes[cursor];
        value |= ((current_byte & SEGMENT_BITS) as i64) << position;
        if current_byte & CONTINUE_BIT == 0 { return value; }
        cursor += 1; position += 7;
        if position >= 64 { panic!("VarInt is too big") }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vari32() {
        let value = i32::MAX;
        let bin = to_vari32(value);
        assert_eq!(vari32_byte_length(value), bin.len());
        assert_eq!(value, from_vari32(&bin));
    }

    #[test]
    fn test_vari64() {
        let value = i64::MAX;
        let bin = to_vari64(value);
        assert_eq!(vari64_byte_length(value), bin.len());
        assert_eq!(value, from_vari64(&bin));
    }
}