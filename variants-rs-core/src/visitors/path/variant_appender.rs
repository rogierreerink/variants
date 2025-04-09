use syn::{Error, Ident, Path, TypePath, visit_mut::VisitMut};

use super::ident_appender::PathIdentAppender;

pub struct PathVariantAppender<'a> {
    ty_path: &'a TypePath,
    variant: &'a Option<Ident>,
    errors: &'a mut Vec<Error>,
}

impl<'a> PathVariantAppender<'a> {
    /// Append `variant` to all `ty_path`s.
    ///
    /// It isn't necessary to modify the path types on a base implementation,
    /// so this visitor does nothing if `self.variant` is `none`.
    ///
    /// See [crate::visitors::path::path_ident] for more info.
    ///
    pub fn new(
        ty_path: &'a TypePath,
        variant: &'a Option<Ident>,
        errors: &'a mut Vec<Error>,
    ) -> Self {
        Self {
            ty_path,
            variant,
            errors,
        }
    }
}

impl VisitMut for PathVariantAppender<'_> {
    fn visit_path_mut(&mut self, node: &mut Path) {
        let variant = match self.variant {
            Some(variant) => variant,
            None => return,
        };

        PathIdentAppender::new(&self.ty_path.path, variant, self.errors).visit_path_mut(node);
    }
}
