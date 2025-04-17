use std::collections::HashMap;

use squattr::{attribute::Attribute, derive::Squattr};
use syn::{Error, FieldValue, Ident, visit_mut::VisitMut};

use crate::utilities::errors_ext::ErrorsExt;

use super::Context;

pub struct FieldValueContext<'a> {
    pub context: &'a Context,
    pub settings: HashMap<Ident, VariantSettings>,
    pub errors: Vec<Error>,
}

impl<'a> FieldValueContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            settings: HashMap::new(),
            errors: Vec::new(),
        }
    }
}

impl VisitMut for FieldValueContext<'_> {
    fn visit_field_value_mut(&mut self, node: &mut FieldValue) {
        self.visit_attributes_mut(&mut node.attrs);
    }

    fn visit_attributes_mut(&mut self, node: &mut Vec<syn::Attribute>) {
        let attributes = match VariantAttribute::extract_from_attributes(node, "variants") {
            Ok(attrs) => attrs,
            Err(error) => {
                self.errors.push(error);
                return;
            }
        };

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
    }
}

#[derive(Squattr, Clone)]
struct VariantAttribute {
    include: Vec<Ident>,
}

#[derive(Clone)]
pub struct VariantSettings {
    pub variant: Ident,
}
