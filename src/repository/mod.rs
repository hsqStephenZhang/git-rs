use std::path::Path;

use walkdir::WalkDir;

use crate::context::GitContext;

pub enum Status {
    Untracked,
    Modified,
    Clean,
}

#[derive(Clone, Debug)]
pub struct Repo<'a> {
    root: File,
    context: &'a GitContext,
}

impl<'a> Repo<'a> {
    pub fn new(path: &Path, context: &'a GitContext) -> Repo<'a> {
        let walker = WalkDir::new(path);
        todo!()
    }
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
    use walkdir::WalkDir;

    #[test]
    fn test() {
        let walker = WalkDir::new("src").into_iter();
        for entry in walker {
            println!("{}", entry.unwrap().path().display());
        }
    }
}
