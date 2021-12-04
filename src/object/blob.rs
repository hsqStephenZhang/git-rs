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
