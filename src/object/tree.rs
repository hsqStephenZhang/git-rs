use super::ObjectMode;

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
