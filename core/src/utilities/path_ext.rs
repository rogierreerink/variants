use syn::{Ident, Path};

use super::ident_ext::IdentExt;

pub trait PathExt: Sized {
    /// Create a new path with the text of an identifier appended to the last path segment of `self`.
    ///
    fn from_appendix(&self, appendix: &Ident) -> Self;
}

impl PathExt for Path {
    fn from_appendix(&self, appendix: &Ident) -> Self {
        let mut self_cpy = self.clone();

        self_cpy
            .segments
            .last_mut()
            .map(|segment| segment.ident = segment.ident.from_appendix(appendix));

        self_cpy
    }
}
