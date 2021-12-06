use std::path::Path;

use crate::context::GitContext;

use super::File;

#[derive(Clone, Debug)]
pub struct Stage<'a> {
    root: File,
    context: &'a GitContext,
}

impl<'a> Stage<'a> {
    pub fn new(path: &Path, context: &'a GitContext) -> Stage<'a> {
        todo!()
    }
}