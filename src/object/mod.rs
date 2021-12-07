pub mod blob;
pub mod commit;
pub mod tree;

pub use blob::Blob;
pub use commit::Commit;
pub use tree::{Tree, TreeEntry};

#[derive(Clone, Debug)]
pub enum Object {
    Blob(Blob),
    Tree(Tree),
    Commit(Commit),
}

/// mode of object, which will be encoded in tree object's content
/// no supporting submodules yet, whose code is 160000
#[derive(Clone)]
pub enum ObjectMode {
    Dir,
    Normal,
    Executable,
    SymbolLink,
}

impl std::fmt::Debug for ObjectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dir => write!(f, "Dir"),
            Self::Normal => write!(f, "Normal"),
            Self::Executable => write!(f, "Executable"),
            Self::SymbolLink => write!(f, "SymbolLink"),
        }
    }
}
impl std::fmt::Display for ObjectMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dir => write!(f, "40000"),
            Self::Normal => write!(f, "100644"),
            Self::Executable => write!(f, "100755"),
            Self::SymbolLink => write!(f, "120000"),
        }
    }
}

impl From<&[u8]> for ObjectMode {
    fn from(raw: &[u8]) -> Self {
        debug_assert!(raw.len() == 6 || (raw.len() == 5 && raw==b"40000"));
        // dbg!(raw);

        match raw {
            b"40000" => Self::Dir,
            b"100644" => Self::Normal,
            b"100755" => Self::Executable,
            b"120000" => Self::SymbolLink,
            _ => unreachable!(),
        }
    }
}

impl Into<Vec<u8>> for &Object {
    fn into(self) -> Vec<u8> {
        match self {
            Object::Blob(blob) => blob.into(),
            Object::Tree(tree) => tree.into(),
            Object::Commit(commit) => commit.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::sha1;

    use super::*;

    #[test]
    fn test_blob_encode() {
        let content = vec![104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 10];
        let b = Blob::new(content);
        let object = Object::Blob(b);
        let uncompressed_content: Vec<u8> = Into::into(&object);
        let content = sha1::encode(&uncompressed_content);

        let target = vec![
            0x78, 0x01, 0x4B, 0xCA, 0xC9, 0x4F, 0x52, 0x30, 0x34, 0x62, 0xC8, 0x48, 0xCD, 0xC9,
            0xC9, 0x57, 0x28, 0xCF, 0x2F, 0xCA, 0x49, 0xE1, 0x02, 0x00, 0x44, 0x11, 0x06, 0x89,
        ];

        assert_eq!(content, target);
    }
}
