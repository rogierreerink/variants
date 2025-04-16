use syn::{Error, Field, ItemStruct, visit_mut::VisitMut};

use super::{Context, field::FieldContext};

pub struct StructContext<'a> {
    pub context: &'a Context,
    pub field_ctxs: Vec<FieldContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> StructContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            field_ctxs: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl VisitMut for StructContext<'_> {
    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        self.visit_fields_mut(&mut node.fields);
    }

    fn visit_field_mut(&mut self, node: &mut Field) {
        let mut field_ctx = FieldContext::new(self.context);
        field_ctx.visit_field_mut(node);
        self.errors.append(&mut field_ctx.errors);
        self.field_ctxs.push(field_ctx);
    }
}
