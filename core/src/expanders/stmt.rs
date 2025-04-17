use syn::{Error, Stmt, visit_mut::VisitMut};

use crate::context::stmt::StmtContext;

use super::Context;

pub struct StmtExpander<'a> {
    context: &'a Context<'a>,
    stmt_ctx: &'a StmtContext<'a>,
    pub print_stmt: bool,
    pub errors: Vec<Error>,
}

impl<'a> StmtExpander<'a> {
    pub fn new(context: &'a Context, stmt_ctx: &'a StmtContext) -> Self {
        Self {
            context,
            stmt_ctx,
            print_stmt: true,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for StmtExpander<'_> {
    fn visit_stmt_mut(&mut self, _node: &mut Stmt) {
        let variant = match self.context.variant {
            Some(variant) => variant,
            None => return,
        };

        let settings = match self.stmt_ctx.settings.get(variant) {
            Some(context) => context,
            None => return,
        };

        self.print_stmt = settings.include;
    }
}
