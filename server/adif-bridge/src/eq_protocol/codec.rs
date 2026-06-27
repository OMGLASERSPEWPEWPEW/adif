use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::io::{Read, Write};

pub fn crc32(data: &[u8], key: u32) -> u32 {
    let mut crc = crc32fast::hash(data);
    crc ^= key;
    crc
}

pub fn append_crc(data: &mut Vec<u8>, key: u32, crc_bytes: u8) {
    if crc_bytes == 0 {
        return;
    }
    let crc = crc32(data, key);
    let crc_be = crc.to_be_bytes();
    let start = 4 - crc_bytes as usize;
    data.extend_from_slice(&crc_be[start..]);
}

pub fn verify_and_strip_crc(data: &mut Vec<u8>, key: u32, crc_bytes: u8) -> bool {
    if crc_bytes == 0 || data.len() < crc_bytes as usize + 2 {
        return true;
    }

    let crc_len = crc_bytes as usize;
    let body = &data[..data.len() - crc_len];
    let crc = crc32(body, key);
    let crc_be = crc.to_be_bytes();

    let start = 4 - crc_len;
    let expected = &crc_be[start..];
    let actual = &data[data.len() - crc_len..];

    if expected != actual {
        return false;
    }

    data.truncate(data.len() - crc_len);
    true
}

pub fn decompress(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let flag = data[0];
    if flag == 0xa5 {
        let mut decoder = ZlibDecoder::new(&data[1..]);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out)?;
        Ok(out)
    } else {
        Ok(data[1..].to_vec())
    }
}

pub fn compress(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    let compressed = encoder.finish().unwrap();

    if compressed.len() < data.len() {
        let mut out = vec![0xa5];
        out.extend_from_slice(&compressed);
        out
    } else {
        let mut out = vec![0x00];
        out.extend_from_slice(data);
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_round_trip() {
        let mut data = vec![0x00, 0x09, 0x00, 0x01, 0x48, 0x65, 0x6c, 0x6c, 0x6f];
        let key = 0x12345678u32;
        append_crc(&mut data, key, 2);
        assert_eq!(data.len(), 11);
        assert!(verify_and_strip_crc(&mut data, key, 2));
        assert_eq!(data.len(), 9);
    }

    #[test]
    fn crc_detects_corruption() {
        let mut data = vec![0x00, 0x09, 0x00, 0x01, 0x48];
        let key = 0xAABBCCDD;
        append_crc(&mut data, key, 2);
        data[3] = 0xFF; // corrupt
        assert!(!verify_and_strip_crc(&mut data, key, 2));
    }

    #[test]
    fn compress_decompress_round_trip() {
        let original = b"Hello, Norrath! This is a test of the compression system.";
        let compressed = compress(original);
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(&decompressed, original);
    }

    #[test]
    fn decompress_uncompressed_flag() {
        let mut data = vec![0x00]; // flag = not compressed
        data.extend_from_slice(b"raw data");
        let result = decompress(&data).unwrap();
        assert_eq!(&result, b"raw data");
    }
}
