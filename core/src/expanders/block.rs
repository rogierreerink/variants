use std::collections::HashMap;

use syn::{
    Block, Error, Stmt,
    visit_mut::{VisitMut, visit_block_mut},
};

use crate::context::stmt::StmtContext;

use super::{Context, stmt::StmtExpander};

pub struct BlockExpander<'a> {
    context: &'a Context<'a>,
    stmt_ctxs: &'a HashMap<Stmt, StmtContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> BlockExpander<'a> {
    pub fn new(context: &'a Context, stmt_ctxs: &'a HashMap<Stmt, StmtContext<'a>>) -> Self {
        Self {
            context,
            stmt_ctxs,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for BlockExpander<'_> {
    fn visit_block_mut(&mut self, node: &mut Block) {
        node.stmts = node
            .stmts
            .iter_mut()
            .filter_map(|stmt| {
                let stmt_ctx = match self.stmt_ctxs.get(stmt) {
                    Some(context) => context,
                    None => return Some(stmt.clone()),
                };

                let mut stmt_expander = StmtExpander::new(self.context, stmt_ctx);
                stmt_expander.visit_stmt_mut(stmt);
                self.errors.append(&mut stmt_expander.errors);

                if stmt_expander.print_stmt {
                    Some(stmt.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        visit_block_mut(self, node);
    }
}
