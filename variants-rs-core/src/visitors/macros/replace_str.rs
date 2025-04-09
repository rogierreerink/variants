use syn::{
    Error, Expr, ExprLit, ExprMacro, Lit, LitStr, Macro,
    spanned::Spanned,
    visit_mut::{VisitMut, visit_expr_mut, visit_macro_mut},
};

pub struct ReplaceStrMacro<'a> {
    macro_name: &'static str,
    replacement_str: String,
    errors: &'a mut Vec<Error>,
}

impl<'a> ReplaceStrMacro<'a> {
    /// Replace all expression macros named `macro_name!()` with a string
    /// literal with the content of `replacement_str`.
    ///
    /// The macros span and attributes are preserved.
    ///
    pub fn new(
        macro_name: &'static str,
        replacement_str: String,
        errors: &'a mut Vec<Error>,
    ) -> Self {
        Self {
            macro_name,
            replacement_str,
            errors,
        }
    }
}

impl VisitMut for ReplaceStrMacro<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        if let Expr::Macro(ExprMacro { attrs, mac }) = node {
            if !mac.path.is_ident(self.macro_name) {
                return;
            }

            *node = Expr::Lit(ExprLit {
                attrs: attrs.to_vec(),
                lit: Lit::Str(LitStr::new(&self.replacement_str, node.span())),
            });
        }

        visit_expr_mut(self, node);
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

        visit_macro_mut(self, node);
    }
}
