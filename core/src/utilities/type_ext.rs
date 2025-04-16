use syn::{Ident, Type, TypePath};

use super::ident_ext::IdentExt;

pub trait TypePathExt: Sized {
    /// Transform an identifier into a type (Type::Path).
    ///
    fn into_type(self) -> Type;

    /// Create a new type path with the text of an identifier appended to the last path segment of
    /// `self`.
    ///
    fn from_appendix(&self, appendix: &Ident) -> Self;
}

impl TypePathExt for TypePath {
    fn into_type(self) -> Type {
        Type::Path(self)
    }

    fn from_appendix(&self, appendix: &Ident) -> Self {
        let mut self_cpy = self.clone();

        self_cpy
            .path
            .segments
            .last_mut()
            .map(|segment| segment.ident = segment.ident.from_appendix(appendix));

        self_cpy
    }
}
