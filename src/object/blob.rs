use bytes::{BufMut, BytesMut};

// blob <content length><NULL><content>
#[derive(Clone, Debug)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}

impl From<&[u8]> for Blob {
    fn from(content: &[u8]) -> Self {
        Self::new(content.into())
    }
}

impl Into<Vec<u8>> for &Blob {
    fn into(self) -> Vec<u8> {
        let length = self.content.len();
        let s=format!("{}",length);
        let mut buf = BytesMut::with_capacity(length + 4);
        buf.put(&b"blob "[..]);
        buf.put(s.as_bytes());
        buf.put_u8(b'\0');
        buf.put(&self.content[..]);
        (&buf[..]).into()
    }
}
