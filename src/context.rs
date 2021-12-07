use std::{fs, path::PathBuf};

use nom::IResult;

use crate::{
    index::Index,
    object::Object,
    parser::decode::{decode_head_pointer, decode_index, decode_object},
    utils::{path::object_path, sha1},
    GitError,
};

/// it should be created at the very beginning, so it's ok to hold it's reference in other structs
///
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GitContext {
    pub head: String,
    pub repo: Object,
    pub index: Index,
    pub workdspace: WorkdSpace,
}

impl GitContext {
    pub fn try_new(path: PathBuf) -> Result<Self, GitError> {
        #[cfg(not(test))]
        let root_path = path.join(crate::REPO_NAME);
        #[cfg(test)]
        let root_path = path.join(".git");

        let head_path = root_path.join("HEAD");
        let head_file_content = fs::read(head_path.clone())
            .map_err(|_e| GitError::CorruptRepo(head_path.to_str().unwrap().into()))?;
        let head: IResult<_, _> = decode_head_pointer(&head_file_content);
        let (_, head) =
            head.map_err(|_e| GitError::CorruptRepo(head_path.to_str().unwrap().into()))?;

        let mut path = match head {
            crate::refs::Head::Ref(reference) => {
                let path = root_path.join(&reference);
                dbg!(&path);
                let content = fs::read(path).unwrap();
                String::from_utf8(content).unwrap()
            }
            crate::refs::Head::Pointer(pointer) => String::from_utf8(pointer).unwrap(),
        };
        path.pop(); // remove the '\n' at the end of file

        let full_path = object_path(&root_path, &path);
        let deflated_content = sha1::decode_file(&full_path);
        dbg!(&path, &full_path);
        let commit_object: IResult<_, _> = decode_object(&deflated_content);
        let commit_object = commit_object.unwrap();
        dbg!(&commit_object);

        let index_content = fs::read(root_path.join("index")).unwrap();
        // let index= Index::try_new();
        let index: IResult<_, _> = decode_index(&index_content);
        let (_, index) = index.unwrap();

        dbg!(&index);

        todo!()
    }
}

#[derive(Clone, Debug)]
pub struct WorkdSpace {}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_build_context() {
        let path: Box<Path> = Path::new(".").into();
        let git_context = GitContext::try_new(path.into_path_buf());
        dbg!(&git_context);
    }
}
