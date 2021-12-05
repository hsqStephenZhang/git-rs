/// when we traverse the git repository, chances are that we need to
/// get file's content(Vec<u8>) very often, so instead of read the entire
/// file content each time, we will cache them in a hashmap.
/// when a function needs to read the content, it should lookup the cache first.

use std::{collections::HashMap, path::PathBuf, sync::Mutex};


lazy_static! {
    pub static ref FILE_CACHE: Mutex<HashMap<PathBuf, Vec<u8>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
    pub static ref SHA_CACHE: Mutex<HashMap<PathBuf, Vec<u8>>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };
}