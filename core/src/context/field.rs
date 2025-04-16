use std::collections::HashMap;

use squattr::{attribute::Attribute, derive::Squattr};
use syn::{Error, Field, Ident, LitStr, visit_mut::VisitMut};

use crate::errors::ErrorsExt;

use super::Context;

pub struct FieldContext<'a> {
    pub context: &'a Context,
    pub settings: HashMap<Ident, VariantSettings>,
    pub errors: Vec<Error>,
}

impl<'a> FieldContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            settings: HashMap::new(),
            errors: Vec::new(),
        }
    }
}

impl VisitMut for FieldContext<'_> {
    fn visit_field_mut(&mut self, node: &mut Field) {
        self.visit_attributes_mut(&mut node.attrs);
    }

    fn visit_attributes_mut(&mut self, node: &mut Vec<syn::Attribute>) {
        let attributes = node
            .iter()
            .filter_map(|attr| {
                if !attr.path().is_ident("variants") {
                    return None;
                }

                match VariantAttribute::from_meta(&attr.meta) {
                    Ok(attr) => Some(attr),
                    Err(error) => {
                        self.errors.push(error);
                        None
                    }
                }
            })
            .collect::<Vec<_>>();

        self.settings = attributes.iter().fold(HashMap::new(), |mut acc, attr| {
            for variant in &attr.include {
                if !self
                    .errors
                    .contains_variant(&self.context.variants, &variant)
                {
                    continue;
                }

                if let Some(_) = acc.insert(
                    variant.clone(),
                    VariantSettings {
                        variant: variant.clone(),
                        retype: attr.retype.clone(),
                    },
                ) {
                    self.errors.push(Error::new(
                        variant.span(),
                        format!("duplicate settings for variant `{}`", variant),
                    ));
                }
            }

            acc
        });

        node.retain(|attribute| !attribute.path().is_ident("variants"));
    }
}

#[derive(Squattr, Clone)]
struct VariantAttribute {
    include: Vec<Ident>,
    retype: Option<LitStr>,
}

#[derive(Clone)]
pub struct VariantSettings {
    pub variant: Ident,
    pub retype: Option<LitStr>,
}
