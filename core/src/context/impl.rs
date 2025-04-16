use std::collections::HashMap;

use syn::{Error, FieldValue, visit_mut::VisitMut};

use super::{Context, field_value::FieldValueContext};

pub struct ImplContext<'a> {
    pub context: &'a Context,
    pub field_value_ctxs: HashMap<FieldValue, FieldValueContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> ImplContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            field_value_ctxs: HashMap::new(),
            errors: Vec::new(),
        }
    }
}

impl VisitMut for ImplContext<'_> {
    fn visit_field_value_mut(&mut self, node: &mut FieldValue) {
        let mut field_value_ctx = FieldValueContext::new(self.context);
        field_value_ctx.visit_field_value_mut(node);
        self.errors.append(&mut field_value_ctx.errors);
        self.field_value_ctxs.insert(node.clone(), field_value_ctx);
    }
}
