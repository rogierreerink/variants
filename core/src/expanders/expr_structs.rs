use std::collections::HashMap;

use syn::{Error, ExprStruct, FieldValue, visit_mut::VisitMut};

use crate::{context::field_value::FieldValueContext, utilities::vec_ext::VecExt};

use super::{Context, field_value::FieldValueExpander};

pub struct ExprStructExpander<'a> {
    context: &'a Context<'a>,
    field_value_ctxs: &'a HashMap<FieldValue, FieldValueContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> ExprStructExpander<'a> {
    pub fn new(
        context: &'a Context,
        field_value_ctxs: &'a HashMap<FieldValue, FieldValueContext<'a>>,
    ) -> Self {
        Self {
            context,
            field_value_ctxs,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for ExprStructExpander<'_> {
    fn visit_expr_struct_mut(&mut self, node: &mut ExprStruct) {
        let print_fields = node
            .fields
            .iter_mut()
            .filter_map(|field_value_node| {
                let field_value_ctx = match self.field_value_ctxs.get(field_value_node) {
                    Some(context) => context,
                    None => return None,
                };

                let mut field_value_expander =
                    FieldValueExpander::new(self.context, field_value_ctx);
                field_value_expander.visit_field_value_mut(field_value_node);
                self.errors.append(&mut field_value_expander.errors);

                if field_value_expander.print_field {
                    Some(field_value_node.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        node.fields = print_fields.into_punctuated();
    }
}
