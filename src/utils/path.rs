//! path related utils
//! we should locate the root directory at the very first beginning
//! and when we exec `git-rs add xxx`, we will walk the whole path,
//! it's also needy to provide some utils for this usage

use std::path::PathBuf;

use crate::REPO_NAME;

pub fn root_dir() -> Result<PathBuf, ()> {
    let mut dir = std::env::current_dir().unwrap();
    if dir.join(REPO_NAME).exists() {
        return Ok(dir);
    }
    while dir.pop() {
        if dir.join(REPO_NAME).exists() {
            return Ok(dir);
        }
    }

    Err(())
}

pub fn object_path(root_path:&PathBuf,name: &str) -> PathBuf
{
    let dir_name = root_path.join("objects").join(&name[0..2]);
    let filename = &name[2..];
    let full_path = dir_name.join(filename);
    full_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        use std::path::Path;

        let path = Path::new("../");
        std::env::set_current_dir(&path).unwrap();
        let root = root_dir();
        assert_eq!(root, Err(()));
    }
}
