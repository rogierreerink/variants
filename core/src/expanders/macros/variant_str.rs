use syn::{Error, Expr, Ident, Macro, visit_mut::VisitMut};

use super::insert_str::InsertStrMacro;

pub struct VariantStrMacro {
    variant_str: String,
    pub errors: Vec<Error>,
}

impl VariantStrMacro {
    const IDENTIFIER: &'static str = "variant_str";

    /// Replace all `variant_str!()` expression macros with a string literal with the content of
    /// `variant`, or an empty string if `variant` is None.
    ///
    pub fn new(variant: &Option<&Ident>) -> Self {
        Self {
            variant_str: match variant {
                Some(variant) => variant.to_string(),
                None => "".into(),
            },
            errors: Vec::new(),
        }
    }
}

impl VisitMut for VariantStrMacro {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        let mut insert_str_macro = InsertStrMacro::new(Self::IDENTIFIER, self.variant_str.clone());
        insert_str_macro.visit_expr_mut(node);
        self.errors.append(&mut insert_str_macro.errors);
    }

    /// Emit an error on non-expression macro invocations.
    ///
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        let mut insert_str_macro = InsertStrMacro::new(Self::IDENTIFIER, self.variant_str.clone());
        insert_str_macro.visit_macro_mut(node);
        self.errors.append(&mut insert_str_macro.errors);
    }
}
