use flate2::Compression;
use std::io::prelude::*;

use std::{io::BufReader, path::Path};

pub fn encode_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    use flate2::bufread::ZlibEncoder;

    let file = std::fs::File::open(path).unwrap();
    let b = BufReader::new(file);
    let mut z = ZlibEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn encode(content: &[u8]) -> Vec<u8> {
    use flate2::write::ZlibEncoder;

    let mut z = ZlibEncoder::new(Vec::new(), Compression::fast());
    z.write_all(&content).unwrap();
    let compressed = z.finish().unwrap();
    compressed
}

pub fn decode_file<P: AsRef<Path>>(path: P) -> Vec<u8> {
    use flate2::bufread::ZlibDecoder;

    let bytes = std::fs::read(path).unwrap();
    let mut deflater = ZlibDecoder::new(&bytes[..]);
    let mut reader = Vec::new();
    deflater.read_to_end(&mut reader).unwrap();
    reader
}

pub fn decode(bytes: &[u8]) -> Vec<u8> {
    use flate2::bufread::ZlibDecoder;

    let mut deflater = ZlibDecoder::new(&bytes[..]);
    let mut reader = Vec::new();
    deflater.read_to_end(&mut reader).unwrap();
    reader
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encode() {
        let r = include_bytes!("../../data/hello.txt");
        println!("{:x?}", &r);
    }

    #[test]
    fn test_decode_blob() {
        let r = include_bytes!("../../data/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad");
        let res  = decode(&r[..]);
        assert_eq!(&res[..], "blob 12\0hello world\n".as_bytes());
    }

    #[test]
    fn test_sha_name() {
        let r = include_bytes!("../../data/hello.txt");
        let r=encode(&r[..]);
        assert_eq!(r,vec![0x78, 0x1, 0xcb, 0x48, 0xcd, 0xc9, 0xc9, 0x57, 0x28, 0xcf, 0x2f, 0xca, 0x49, 0xe1, 0x2, 0x0, 0x1e, 0x72, 0x4, 0x67]);
    }
}
