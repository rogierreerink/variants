use proc_macro2::Span;
use syn::{Error, Ident, Item, ItemImpl, ItemStruct, Type, spanned::Spanned, visit_mut::VisitMut};

use crate::visitors::macros::{variant_str::VariantStrMacro, variant_type::VariantTypeMacro};

use super::{implementation::ImplVisitor, structure::StructVisitor};

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
        VariantStrMacro::new(self.variant, self.errors).visit_item_mut(node);
        VariantTypeMacro::new(self.variant, self.errors).visit_item_mut(node);

        match node {
            Item::Impl(ItemImpl { self_ty, .. }) => {
                ImplVisitor::new(
                    match self_ty.as_ref() {
                        Type::Path(base) => base.clone(),
                        ty => {
                            self.errors.push(Error::new(
                                ty.span(),
                                "type not supported in implementation",
                            ));
                            return;
                        }
                    },
                    self.variant,
                    self.errors,
                )
                .visit_item_mut(node);
            }
            Item::Struct(ItemStruct { ident, .. }) => {
                StructVisitor::new(ident.clone(), self.variant, self.errors).visit_item_mut(node);
            }
            _ => {
                self.errors.push(Error::new(
                    Span::call_site(),
                    "item type not supported, use on structs \
                    and implementations (or submit a feature request)",
                ));
            }
        }
    }
}
