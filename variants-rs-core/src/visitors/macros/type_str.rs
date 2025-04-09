use syn::{
    Error, Expr, Ident, Macro, TypePath,
    visit_mut::{VisitMut, visit_expr_mut, visit_macro_mut},
};

use super::replace_str::ReplaceStrMacro;

pub struct TypeStrMacro<'a> {
    ty_str: String,
    errors: &'a mut Vec<Error>,
}

impl<'a> TypeStrMacro<'a> {
    const IDENTIFIER: &'static str = "type_str";

    /// Replace all `type_str!()` expression macros with a string literal
    /// that is a combination of `base_path` and `variant`.
    ///
    /// The `variant` string value is append to the last segment of `base_path`
    /// to form the replacement string. If `variant` is `None`, just the last
    /// segment of `base_path` is used.
    ///
    /// The macros span and attributes are preserved.
    ///
    pub fn new(base_path: &TypePath, variant: &Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        let base_str = base_path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string())
            .unwrap_or("".into());

        let variant_str = match variant {
            Some(variant) => variant.to_string(),
            None => "".into(),
        };

        Self {
            ty_str: format!("{}{}", base_str, variant_str),
            errors,
        }
    }
}

impl VisitMut for TypeStrMacro<'_> {
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        ReplaceStrMacro::new(Self::IDENTIFIER, self.ty_str.clone(), self.errors)
            .visit_expr_mut(node);

        visit_expr_mut(self, node);
    }

    /// Emit an error on non-expression macro invocations.
    ///
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        ReplaceStrMacro::new(Self::IDENTIFIER, self.ty_str.clone(), self.errors)
            .visit_macro_mut(node);

        visit_macro_mut(self, node);
    }
}
