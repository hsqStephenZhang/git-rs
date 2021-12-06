pub mod repository;
pub mod stage;
pub mod working_area;

use std::path::Path;

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
    Committed,
}

#[derive(Clone, Debug)]
pub enum File {
    RawFile(RawFile),
    Dir(Dir),
}

#[derive(Clone, Debug)]
pub struct RawFile {
    name: String,
    content: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Dir {
    name: String,
    children: Option<Vec<File>>,
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::path::{Path, PathBuf};

    use std::fs;

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
                    println!("{:?}", dir.path());
                }
            }
        }
    }
}
