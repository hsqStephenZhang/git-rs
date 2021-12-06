use std::{collections::HashMap, path::PathBuf};

/// it should be created at the very beginning, so it's ok to hold it's reference in other structs
/// 
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GitContext {
    pub(crate) raw_cache: HashMap<PathBuf, Vec<u8>>,
    pub(crate) sha1_cache: HashMap<PathBuf, Vec<u8>>,
}
