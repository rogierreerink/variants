use syn::{
    Error, Ident, ItemImpl, Path, TypePath,
    visit_mut::{VisitMut, visit_item_impl_mut, visit_path_mut},
};

use super::{macros::type_str::TypeStrMacro, path::PathAppender};

pub struct ImplVisitor<'a> {
    ty_path: TypePath,
    variant: &'a Option<Ident>,
    errors: &'a mut Vec<Error>,
}

impl<'a> ImplVisitor<'a> {
    pub fn new(ty_path: TypePath, variant: &'a Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self {
            ty_path,
            variant,
            errors,
        }
    }
}

impl VisitMut for ImplVisitor<'_> {
    /// Entry point for traversing an implementation block.
    ///
    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        TypeStrMacro::new(&self.ty_path, &self.variant, self.errors).visit_item_impl_mut(node);

        visit_item_impl_mut(self, node);
    }

    /// Append `self.variant` to the structs path type nodes.
    ///
    /// It isn't necessary to modify the path types on a base implementation,
    /// so this visitor does nothing if `self.variant` is `none`.
    ///
    /// See [crate::visitors::path] for more info.
    ///
    fn visit_path_mut(&mut self, node: &mut Path) {
        let variant = match self.variant {
            Some(variant) => variant,
            None => return,
        };

        PathAppender::new(&self.ty_path.path, variant, self.errors).visit_path_mut(node);

        visit_path_mut(self, node);
    }
}
