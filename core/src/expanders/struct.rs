use syn::{Error, ItemStruct, visit_mut::VisitMut};

use crate::{
    context::r#struct::StructContext,
    utilities::{fields_ext::FieldsExt, ident_ext::IdentExt},
};

use super::{Context, field::FieldExpander};

pub struct StructExpander<'a> {
    context: &'a Context<'a>,
    struct_ctx: &'a StructContext<'a>,
    pub errors: Vec<Error>,
}

impl<'a> StructExpander<'a> {
    pub fn new(context: &'a Context, struct_ctx: &'a StructContext<'a>) -> Self {
        Self {
            context,
            struct_ctx,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for StructExpander<'_> {
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let print_fields = node
            .fields
            .iter_mut()
            .zip(self.struct_ctx.field_ctxs.iter())
            .filter_map(|(field_node, field_ctx)| {
                let mut field_expander = FieldExpander::new(self.context, &node.ident, &field_ctx);
                field_expander.visit_field_mut(field_node);
                self.errors.append(&mut field_expander.errors);

                if field_expander.print_field {
                    Some(field_node.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        node.fields.replace_fields(print_fields);

        if let Some(variant) = self.context.variant {
            node.ident = node.ident.from_appendix(variant);
        }
    }
}
