use syn::{Error, FieldValue, visit_mut::VisitMut};

use crate::context::field_value::FieldValueContext;

use super::Context;

pub struct FieldValueExpander<'a> {
    context: &'a Context<'a>,
    field_value_ctx: &'a FieldValueContext<'a>,
    pub print_field: bool,
    pub errors: Vec<Error>,
}

impl<'a> FieldValueExpander<'a> {
    pub fn new(context: &'a Context, field_value_ctx: &'a FieldValueContext) -> Self {
        Self {
            context,
            field_value_ctx,
            print_field: true,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for FieldValueExpander<'_> {
    fn visit_field_value_mut(&mut self, _node: &mut FieldValue) {
        let variant = match self.context.variant {
            Some(variant) => variant,
            None => return,
        };

        let settings = match self.field_value_ctx.settings.get(variant) {
            Some(context) => context,
            None => {
                self.print_field = false;
                return;
            }
        };

        if &settings.variant != variant {
            self.print_field = false;
        }
    }
}
