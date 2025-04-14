use syn::{
    Error, Expr, ExprLit, ExprMacro, Lit, LitStr, Macro, spanned::Spanned, visit_mut::VisitMut,
};

pub struct InsertStrMacro<'a> {
    macro_name: &'static str,
    insert_str: String,
    errors: &'a mut Vec<Error>,
}

impl<'a> InsertStrMacro<'a> {
    /// Replace all expression macros named `macro_name!()` with string
    /// literal `insert_str`.
    ///
    /// The macros span and attributes are preserved.
    ///
    pub fn new(macro_name: &'static str, insert_str: String, errors: &'a mut Vec<Error>) -> Self {
        Self {
            macro_name,
            insert_str,
            errors,
        }
    }
}

impl VisitMut for InsertStrMacro<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Macro(ExprMacro { attrs, mac }) = node {
            if !mac.path.is_ident(self.macro_name) {
                return;
            }

            *node = Expr::Lit(ExprLit {
                attrs: attrs.to_vec(),
                lit: Lit::Str(LitStr::new(&self.insert_str, node.span())),
            });
        }
    }

    /// Emit an error on non-expression macro invocations.
    ///
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        if !node.path.is_ident(self.macro_name) {
            return;
        }

        self.errors.push(Error::new(
            node.span(),
            format!("`{}!()` may only be used in expressions", self.macro_name),
        ));
    }
}
