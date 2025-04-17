use syn::{Error, ItemEnum, Variant, visit_mut::VisitMut};

use crate::{
    context::r#enum::EnumContext,
    utilities::{fields_ext::FieldsExt, ident_ext::IdentExt},
};

use super::{Context, field::FieldExpander};

pub struct EnumExpander<'a> {
    context: &'a Context<'a>,
    enum_ctx: &'a EnumContext<'a>,
    pub errors: Vec<Error>,
}

impl<'a> EnumExpander<'a> {
    pub fn new(context: &'a Context, enum_ctx: &'a EnumContext<'a>) -> Self {
        Self {
            context,
            enum_ctx,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for EnumExpander<'_> {
    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        for variant in &mut node.variants {
            self.visit_variant_mut(variant);
        }

        if let Some(variant) = self.context.variant {
            node.ident = node.ident.from_appendix(variant);
        }
    }

    fn visit_variant_mut(&mut self, node: &mut Variant) {
        let print_fields = node
            .fields
            .iter_mut()
            .filter_map(|field| {
                let field_ctx = match self.enum_ctx.field_ctxs.get(field) {
                    Some(context) => context,
                    None => return None,
                };

                let mut field_expander = FieldExpander::new(self.context, &node.ident, &field_ctx);
                field_expander.visit_field_mut(field);
                self.errors.append(&mut field_expander.errors);

                if field_expander.print_field {
                    Some(field.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        node.fields.replace_fields(print_fields);
    }
}
