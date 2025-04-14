use squattr::{attribute::Attribute, derive::Squattr};
use syn::{
    Error, Field, Ident, ItemStruct, LitStr,
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
        let attrs = node
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("variants"))
            .filter_map(|attr| match FieldAttribute::from_meta(&attr.meta) {
                Ok(parsed) => Some(parsed),
                Err(error) => {
                    self.errors.push(error.into());
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("{:#?}\n", attrs);
    }
}

#[derive(Squattr, Debug)]
struct FieldAttribute {
    include: Vec<Ident>,
    retype: Option<LitStr>,
}
