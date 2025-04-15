use std::collections::HashMap;

use quote::quote;
use squattr::{attribute::Attribute, derive::Squattr};
use syn::{
    Error, Field, Ident, ItemStruct, LitStr, Type,
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

    fn field_settings(&mut self, attributes: &mut Vec<syn::Attribute>) -> Option<FieldSettings> {
        let field_attributes = attributes
            .iter()
            .filter(|attribute| attribute.path().is_ident("variants"))
            .filter_map(
                |attribute| match FieldAttribute::from_meta(&attribute.meta) {
                    Ok(parsed) => Some(parsed),
                    Err(error) => {
                        self.errors.push(error.into());
                        None
                    }
                },
            )
            .collect::<Vec<_>>();

        // Remove all `variants` attributes from the node
        attributes.retain_mut(|attribute| !attribute.path().is_ident("variants"));

        let variant = match &self.variant {
            Some(variant) => variant,
            None => return None,
        };

        let settings = field_attributes
            .iter()
            .fold(HashMap::new(), |mut acc, attribute| {
                for variant in &attribute.include {
                    if let Some(_) = acc.insert(
                        variant,
                        FieldSettings {
                            retype: attribute.retype.clone(),
                        },
                    ) {
                        self.errors.push(Error::new(
                            variant.span(),
                            format!("duplicate field settings for variant `{}`", variant),
                        ));
                        continue;
                    }
                }
                acc
            })
            .get(variant)
            .cloned();

        settings
    }

    fn retype_field(&mut self, node_ty: &mut Type, retype: &LitStr) {
        let ty_str = quote!(#node_ty).to_string();
        let retyped_str = retype
            .value()
            .replace("{}", &ty_str)
            .replace("{t}", &ty_str)
            .replace(
                "{v}",
                &self
                    .variant
                    .as_ref()
                    .map(|variant| variant.to_string())
                    .unwrap_or("".into()),
            );

        match LitStr::new(&retyped_str, retype.span()).parse::<Type>() {
            Ok(retyped) => *node_ty = retyped,
            Err(error) => {
                self.errors.push(syn::Error::new(retype.span(), error));
            }
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
        let settings = match self.field_settings(&mut node.attrs) {
            Some(settings) => settings,
            None => return,
        };

        if let Some(retype) = settings.retype {
            self.retype_field(&mut node.ty, &retype)
        }
    }
}

#[derive(Squattr, Debug)]
struct FieldAttribute {
    include: Vec<Ident>,
    retype: Option<LitStr>,
}

#[derive(Clone, Debug)]
struct FieldSettings {
    retype: Option<LitStr>,
}
