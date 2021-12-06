use std::path::Path;

use crate::context::GitContext;

use super::File;

#[derive(Clone, Debug)]
pub struct Repository<'a> {
    root: File,
    context: &'a GitContext,
}

impl<'a> Repository<'a> {
    pub fn new(path: &Path, context: &'a GitContext) -> Repository<'a> {
        todo!()
    }
}