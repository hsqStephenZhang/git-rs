use thiserror::Error;

///! currently, we will ignore the errors with unwrap
#[derive(Error, Debug)]
pub enum GitError {
    #[error("create `{0}` without permission")]
    PermissionDenied(String),
    #[error("repo corrupt, please check `{0}`")]
    CorruptRepo(String),
    #[error("unknown data store error")]
    Unknown,
}