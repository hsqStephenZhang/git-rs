use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
};

use crate::{context::GitContext, core::RawFile};

use super::{Dir, File};

#[derive(Clone, Debug)]
pub struct WorkingArea {
    root: File,
}

impl WorkingArea {
    pub fn new(tree_root: PathBuf) -> WorkingArea {
        let root = add_child(&tree_root);
        Self { root }
    }
}

pub fn add_child(path: &PathBuf) -> File {
    if path.is_dir() {
        let res = (fs::read_dir(&path).unwrap())
            .into_iter()
            .map(|entry| {
                let dir = entry.unwrap();
                let path = dir.path();
                let child = add_child(&path);
                child
            })
            .collect();
        let dir = Dir {
            name: path.clone(),
            children: res,
        };

        return File::Dir(dir);
    } else {
        let content = fs::read(&path).unwrap();
        let raw_file = RawFile {
            name: path.clone(),
            content,
        };
        return File::RawFile(raw_file);
    }
}

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn test(){
        let path = Path::new("src");
        let working_area= WorkingArea::new(path.to_path_buf());
        dbg!(working_area);
    }
}