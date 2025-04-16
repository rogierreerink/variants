use quote::quote;
use syn::{Error, Field, Ident, LitStr, Type, visit_mut::VisitMut};

use crate::context::field::FieldContext;

use super::Context;

pub struct FieldExpander<'a> {
    context: &'a Context<'a>,
    struct_base_ident: &'a Ident,
    field_ctx: &'a FieldContext<'a>,
    pub print_field: bool,
    pub errors: Vec<Error>,
}

impl<'a> FieldExpander<'a> {
    pub fn new(
        context: &'a Context,
        struct_base_ident: &'a Ident,
        field_ctx: &'a FieldContext,
    ) -> Self {
        Self {
            context,
            struct_base_ident,
            field_ctx,
            print_field: true,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for FieldExpander<'_> {
    fn visit_field_mut(&mut self, node: &mut Field) {
        let variant = match self.context.variant {
            Some(variant) => variant,
            None => return,
        };

        let settings = match self.field_ctx.settings.get(variant) {
            Some(settings) => settings,
            None => {
                self.print_field = false;
                return;
            }
        };

        if let Some(retype) = &settings.retype {
            let ty = &node.ty;
            let ty_str = quote!(#ty).to_string();

            let retyped_str = retype
                .value()
                .replace("{}", &ty_str)
                .replace("{t}", &ty_str)
                .replace("{b}", &self.struct_base_ident.to_string())
                .replace("{v}", &variant.to_string());

            match LitStr::new(&retyped_str, retype.span()).parse::<Type>() {
                Ok(retyped) => node.ty = retyped,
                Err(error) => self.errors.push(error),
            }
        }
    }
}
