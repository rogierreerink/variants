use proc_macro2::Span;
use syn::{Error, Ident, Item, ItemImpl, Type, visit_mut::VisitMut};

use super::{implement::ImplVisitor, macros::variant_str::VariantStrMacro};

pub struct ItemVisitor<'a> {
    errors: &'a mut Vec<Error>,
    variant: &'a Option<Ident>,
}

impl<'a> ItemVisitor<'a> {
    pub fn new(variant: &'a Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self { variant, errors }
    }
}

impl VisitMut for ItemVisitor<'_> {
    /// Entry point for traversing the supported input items.
    ///
    fn visit_item_mut(&mut self, node: &mut Item) {
        VariantStrMacro::new(&self.variant, self.errors).visit_item_mut(node);

        match node {
            Item::Impl(ItemImpl { self_ty, .. }) => {
                // I don't think it's possible for an implementation to be made
                // for types other than Type::Path.
                let ty_path = if let Type::Path(base) = self_ty.as_ref() {
                    base.clone()
                } else {
                    self.errors.push(syn::Error::new(
                        Span::call_site(),
                        "impls on types other than `Type::Path` are not supported",
                    ));
                    return;
                };

                ImplVisitor::new(ty_path, self.variant, self.errors).visit_item_mut(node)
            }
            _ => {}
        }
    }
}
