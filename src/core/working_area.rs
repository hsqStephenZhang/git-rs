use std::path::Path;

use crate::context::GitContext;

use super::File;

#[derive(Clone, Debug)]
pub struct WorkingArea<'a> {
    root: File,
    context: &'a GitContext,
}

impl<'a> WorkingArea<'a> {
    pub fn new(path: &Path, context: &'a GitContext) -> WorkingArea<'a> {
        todo!()
    }
}