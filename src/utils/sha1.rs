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

pub fn encode(content: Vec<u8>) -> Vec<u8> {
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

pub fn decode(bytes: Vec<u8>) -> Vec<u8> {
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
        let r = encode_file("data/hello.txt");
        println!("{:x?}", &r);
    }

    #[test]
    fn test_decode_blob() {
        let r = decode_file(".git/objects/3b/18e512dba79e4c8300dd08aeb37f8e728b8dad");
        println!("{:x?}", &r);
        assert_eq!(&r[..], "blob 12\0hello world\n".as_bytes());
    }

    #[test]
    fn test_decode_commit() {
        let r = decode_file(".git/objects/77/a98a3098531ee305c021302cd381386aa41bcd");
        println!("{:x?}", &r);
    }

    #[test]
    fn test_decode_tree() {
        let r = decode_file(".git/objects/84/65cd187d9bad9e5a7931c2119f16311f9923a7");
        println!("{:x?}", &r);
    }

    #[test]
    fn test_sha_name() {
        let r = encode_file("data/hello.txt");
        println!("{:x?}", &r);
    }
}
