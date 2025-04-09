use syn::{
    Error, Expr, Ident, Macro,
    visit_mut::{VisitMut, visit_expr_mut, visit_macro_mut},
};

use super::replace_str::ReplaceStrMacro;

pub struct VariantStrMacro<'a> {
    variant_str: String,
    errors: &'a mut Vec<Error>,
}

impl<'a> VariantStrMacro<'a> {
    const IDENTIFIER: &'static str = "variant_str";

    /// Replace all `variant_str!()` expression macros with a string literal
    /// with the content of `variant`, or an empty string if `variant` is None.
    ///
    /// The macros span and attributes are preserved.
    ///
    pub fn new(variant: &Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self {
            variant_str: match variant {
                Some(variant) => variant.to_string(),
                None => "".into(),
            },
            errors,
        }
    }
}

impl VisitMut for VariantStrMacro<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        ReplaceStrMacro::new(Self::IDENTIFIER, self.variant_str.clone(), self.errors)
            .visit_expr_mut(node);

        visit_expr_mut(self, node);
    }

    /// Emit an error on non-expression macro invocations.
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        ReplaceStrMacro::new(Self::IDENTIFIER, self.variant_str.clone(), self.errors)
            .visit_macro_mut(node);

        visit_macro_mut(self, node);
    }
}
