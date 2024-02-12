use std::io::{Write, Read};

use flate2::{write::ZlibEncoder, Compression, read::ZlibDecoder};

pub fn compress(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    let compressed_data = encoder.finish()?;
    Ok(compressed_data)
}

pub fn decompress(compressed: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(compressed);
    let mut decompressed_data = vec![];
    decoder.read_to_end(&mut decompressed_data)?;
    Ok(decompressed_data)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{compress, decompress};

    #[test]
    fn test_compression() {
        let data = fs::read_to_string("/home/zirkonya/.workspace/rust/zirconium/zirconium_mc_lib/src/tools/utils/compress.rs").expect("Error while getting file");
        let compressed = compress(data.as_bytes()).expect("Error while compressed");
        let decompressed = decompress(&compressed).expect("Error while uncompressed");
        assert_eq!(data.as_bytes(), decompressed);
    }
}