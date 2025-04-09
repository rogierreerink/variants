use syn::{Error, Ident, Item, ItemImpl, Type, spanned::Spanned, visit_mut::VisitMut};

use super::{implement::ImplVisitor, macros::variant_str::VariantStrMacro};

pub struct ItemVisitor<'a> {
    errors: &'a mut Vec<Error>,
    variant: &'a Option<Ident>,
}

impl<'a> ItemVisitor<'a> {
    /// Traverse supported input items.
    ///
    pub fn new(variant: &'a Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self { variant, errors }
    }
}

impl VisitMut for ItemVisitor<'_> {
    fn visit_item_mut(&mut self, node: &mut Item) {
        VariantStrMacro::new(&self.variant, self.errors).visit_item_mut(node);

        match node {
            Item::Impl(ItemImpl { self_ty, .. }) => {
                let ty_path = match self_ty.as_ref() {
                    Type::Path(base) => base.clone(),
                    ty => {
                        Error::new(ty.span(), "type not supported in implementation");
                        return;
                    }
                };

                ImplVisitor::new(ty_path, self.variant, self.errors).visit_item_mut(node)
            }
            _ => {}
        }
    }
}
