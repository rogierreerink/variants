use syn::{Ident, Path, PathArguments, PathSegment, Type, TypePath, punctuated::Punctuated};

pub trait IdentExt: Sized {
    /// Transform an identifier into a path.
    ///
    fn into_path(self) -> Path;

    /// Transform an identifier into a type path.
    ///
    fn into_type_path(self) -> TypePath {
        TypePath {
            qself: None,
            path: self.into_path(),
        }
    }

    /// Transform an identifier into a type (Type::Path).
    ///
    fn into_type(self) -> Type {
        Type::Path(self.into_type_path())
    }
}

impl IdentExt for Ident {
    fn into_path(self) -> Path {
        let mut segments = Punctuated::new();

        segments.push(PathSegment {
            ident: self,
            arguments: PathArguments::None,
        });

        Path {
            leading_colon: None,
            segments,
        }
    }
}
