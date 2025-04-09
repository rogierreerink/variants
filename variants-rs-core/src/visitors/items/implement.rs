use syn::{Error, Ident, ItemImpl, TypePath, visit_mut::VisitMut};

use crate::visitors::{macros::type_str::TypeStrMacro, path::path_variant::PathVariantAppender};

pub struct ImplVisitor<'a> {
    ty_path: TypePath,
    variant: &'a Option<Ident>,
    errors: &'a mut Vec<Error>,
}

impl<'a> ImplVisitor<'a> {
    /// Traverse implementation blocks.
    ///
    pub fn new(ty_path: TypePath, variant: &'a Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self {
            ty_path,
            variant,
            errors,
        }
    }
}

impl VisitMut for ImplVisitor<'_> {
    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        TypeStrMacro::new(&self.ty_path, self.variant, self.errors).visit_item_impl_mut(node);

        PathVariantAppender::new(&self.ty_path, self.variant, self.errors)
            .visit_item_impl_mut(node);
    }
}
