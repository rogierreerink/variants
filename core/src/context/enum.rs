use std::collections::HashMap;

use syn::{Error, Field, Variant, visit_mut::VisitMut};

use super::{Context, field::FieldContext};

pub struct EnumContext<'a> {
    pub context: &'a Context,
    pub field_ctxs: HashMap<Field, FieldContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> EnumContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            field_ctxs: HashMap::new(),
            errors: Vec::new(),
        }
    }
}

impl VisitMut for EnumContext<'_> {
    fn visit_variant_mut(&mut self, node: &mut Variant) {
        self.visit_fields_mut(&mut node.fields);
    }

    fn visit_field_mut(&mut self, node: &mut Field) {
        let mut field_ctx = FieldContext::new(self.context);
        field_ctx.visit_field_mut(node);
        self.errors.append(&mut field_ctx.errors);
        self.field_ctxs.insert(node.clone(), field_ctx);
    }
}
