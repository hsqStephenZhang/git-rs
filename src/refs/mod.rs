#[derive(Clone, Debug)]
pub enum Head {
    Ref(String),
    Pointer(Vec<u8>),
}
