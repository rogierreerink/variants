use std::collections::HashMap;

use syn::{
    Error, FieldValue, Stmt,
    visit_mut::{VisitMut, visit_stmt_mut},
};

use super::{Context, field_value::FieldValueContext, stmt::StmtContext};

pub struct ImplContext<'a> {
    pub context: &'a Context,
    pub field_value_ctxs: HashMap<FieldValue, FieldValueContext<'a>>,
    pub stmt_ctxs: HashMap<Stmt, StmtContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> ImplContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            field_value_ctxs: HashMap::new(),
            stmt_ctxs: HashMap::new(),
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

    fn visit_stmt_mut(&mut self, node: &mut Stmt) {
        visit_stmt_mut(self, node);

        let mut stmt_ctx = StmtContext::new(self.context);
        stmt_ctx.visit_stmt_mut(node);
        self.errors.append(&mut stmt_ctx.errors);
        self.stmt_ctxs.insert(node.clone(), stmt_ctx);
    }
}
