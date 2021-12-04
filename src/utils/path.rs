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

#[cfg(test)]
mod tests {
    use super::*;

    // we can only set current_dir once, otherwise the second shot will fail
    #[test]
    fn test1() {
        use std::path::Path;

        let dir=std::env::current_dir().unwrap();

        let path = Path::new("data/src/test1");
        std::env::set_current_dir(&path).unwrap();
        let root = root_dir();
        assert_eq!(root, Ok(dir));
    }

    #[test]
    fn test2() {
        use std::path::Path;

        let path = Path::new("../");
        std::env::set_current_dir(&path).unwrap();
        let root = root_dir();
        assert_eq!(root, Err(()));
    }
}
