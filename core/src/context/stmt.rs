use std::collections::HashMap;

use squattr::{attribute::Attribute, derive::Squattr};
use syn::{Error, Ident, Local, visit_mut::VisitMut};

use crate::utilities::errors_ext::ErrorsExt;

use super::Context;

pub struct StmtContext<'a> {
    pub context: &'a Context,
    pub settings: HashMap<Ident, VariantSettings>,
    pub errors: Vec<Error>,
}

impl<'a> StmtContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            settings: HashMap::new(),
            errors: Vec::new(),
        }
    }
}

impl VisitMut for StmtContext<'_> {
    fn visit_local_mut(&mut self, node: &mut Local) {
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
            if let Some(include) = &attr.include {
                for variant in include {
                    if !self
                        .errors
                        .contains_variant(&self.context.variants, variant)
                    {
                        continue;
                    }

                    if let Some(_) = acc.insert(
                        variant.clone(),
                        VariantSettings {
                            variant: variant.clone(),
                            include: true,
                        },
                    ) {
                        self.errors.push(Error::new(
                            variant.span(),
                            format!("duplicate settings for variant `{}`", variant),
                        ));
                    }
                }
            }

            if let Some(exclude) = &attr.exclude {
                for variant in exclude {
                    if !self
                        .errors
                        .contains_variant(&self.context.variants, variant)
                    {
                        continue;
                    }

                    if let Some(_) = acc.insert(
                        variant.clone(),
                        VariantSettings {
                            variant: variant.clone(),
                            include: false,
                        },
                    ) {
                        self.errors.push(Error::new(
                            variant.span(),
                            format!("duplicate settings for variant `{}`", variant),
                        ));
                    }
                }
            }

            acc
        });
    }
}

#[derive(Squattr, Clone, Debug)]
struct VariantAttribute {
    include: Option<Vec<Ident>>,
    exclude: Option<Vec<Ident>>,
}

#[derive(Clone, Debug)]
pub struct VariantSettings {
    pub variant: Ident,
    pub include: bool,
}
