use bytes::{BufMut, BytesMut};
use nom::AsBytes;

use crate::utils::bytes::hex_to_bytes;

use super::ObjectMode;

// tree <content length><NUL><file mode> <filename><NUL><item sha>...
#[derive(Clone, Debug)]
pub struct Tree {
    pub entrys: Vec<TreeEntry>,
}

impl Tree {
    pub fn new(entrys: Vec<TreeEntry>) -> Self {
        Self { entrys }
    }
}

#[derive(Clone, Debug)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

#[derive(Clone, Debug)]
pub struct TreeEntry {
    pub mode: ObjectMode,
    pub sha1: String,
    pub filename: String,
}

impl TreeEntry {
    pub fn new(mode: ObjectMode, sha1: String, filename: String) -> Self {
        Self {
            mode,
            sha1,
            filename,
        }
    }
}

impl Into<Vec<u8>> for &Tree {
    fn into(self) -> Vec<u8> {
        let mut entry_buf = BytesMut::with_capacity(1024);

        for entry in self.entrys.iter() {
            let mode = format!("{}", entry.mode);
            entry_buf.put(mode.as_bytes());
            entry_buf.put_u8(b' ');
            entry_buf.put(entry.filename.as_bytes());
            entry_buf.put_u8(b'\0');
            entry_buf.put(hex_to_bytes(entry.sha1.as_bytes()).as_bytes());
        }
        // tree<space><content length><NULL><content>
        // here, content is the entry buf

        let entry_contents = &entry_buf[..];
        let length = entry_contents.len();
        let s = format!("{}", length);
        let mut buf = BytesMut::with_capacity(length + 6 + 10);
        buf.put(&b"tree "[..]);
        buf.put(s.as_bytes());
        buf.put_u8(b'\0');
        buf.put(entry_contents);
        (&buf[..]).into()
    }
}
