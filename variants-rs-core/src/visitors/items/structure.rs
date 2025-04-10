use syn::{
    Error, Field, Ident, ItemStruct,
    visit_mut::{VisitMut, visit_item_struct_mut},
};

use crate::{
    utilities::ident_ext::IdentExt, visitors::path::variant_appender::PathVariantAppender,
};

pub struct StructVisitor<'a> {
    ident: Ident,
    variant: &'a Option<Ident>,
    errors: &'a mut Vec<Error>,
}

impl<'a> StructVisitor<'a> {
    /// Traverse structs.
    ///
    pub fn new(ident: Ident, variant: &'a Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self {
            ident,
            variant,
            errors,
        }
    }
}

impl VisitMut for StructVisitor<'_> {
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        if let Some(variant) = self.variant {
            node.ident = Ident::new(&format!("{}{}", node.ident, variant), variant.span());
        }

        PathVariantAppender::new(
            &self.ident.clone().into_type_path(),
            self.variant,
            self.errors,
        )
        .visit_item_struct_mut(node);

        visit_item_struct_mut(self, node);
    }

    fn visit_field_mut(&mut self, node: &mut Field) {
        println!("other: {:#?}\n", node.attrs);
    }
}
