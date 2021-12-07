pub mod repository;
pub mod stage;
pub mod working_area;

use std::{
    fmt::Debug,
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::context::GitContext;

/**
 *
 *         untracked          unmodified(committed)           modified          staged    
 *             |                     |                            |                 |
 *          add file -------------------------------------------------------------->
 *             |                     |                            |                 |
 *                               edit file ---------------------->                  |
 *             |                     |                            |                 |
 *             |                     |                       stage file ----------->
 *             |                     |                            |                 |
 *              <------------ - remove file
 *             |                     |                            |                 |
 *             |                      <--------------------------------------  commit file
 *             
 */
pub enum Status {
    Untracked,
    Staged,
    Modified,
    StagedAndModified, // the status of file in 3 trees differ from each other
    Committed,
}

#[derive(Clone, Debug)]
pub enum File {
    RawFile(RawFile),
    Dir(Dir),
}

#[derive(Clone)]
pub struct RawFile {
    pub(crate) name: PathBuf,
    pub(crate) content: Vec<u8>,
}

impl Debug for RawFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(debug_content)]
        return f.debug_struct("RawFile").field("name", &self.name).field("content", &self.content).finish();
        #[cfg(not(debug_content))]
        return f.debug_struct("RawFile").field("name", &self.name).finish();
    }
}

#[derive(Clone)]
pub struct Dir {
    pub(crate) name: PathBuf,
    pub(crate) children: Vec<File>,
}

impl Debug for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Dir")
            .field("name", &self.name)
            .field("children", &self.children)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_walk_dir() {
        let tree_root = PathBuf::from("/root/rusty/git-rs/src");

        let mut stack = VecDeque::new();
        stack.push_back(tree_root);

        while stack.len() > 0 {
            let path = stack.pop_front().unwrap();
            for entry in fs::read_dir(&path).unwrap() {
                let dir = entry.unwrap();
                let path = dir.path();
                if path.is_dir() {
                    stack.push_back(path);
                } else {
                    let content = fs::read(&path).unwrap();
                    let raw_file = RawFile {
                        name: path,
                        content,
                    };
                    println!("{:?}", dir.path());
                }
            }
        }
    }
}
