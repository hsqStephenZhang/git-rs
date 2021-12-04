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
            Self::Dir => write!(f, "040000"),
            Self::Normal => write!(f, "100664"),
            Self::Executable => write!(f, "100755"),
            Self::SymbolLink => write!(f, "120000"),
        }
    }
}

impl From<&[u8]> for ObjectMode {
    fn from(raw: &[u8]) -> Self {
        debug_assert!(raw.len() == 6);

        match raw {
            b"040000" => Self::Dir,
            b"100664" => Self::Normal,
            b"100755" => Self::Executable,
            b"120000" => Self::SymbolLink,
            _ => unreachable!(),
        }
    }
}
