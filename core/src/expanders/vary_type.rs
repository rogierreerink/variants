use syn::{Error, ExprStruct, Ident, Path, visit_mut::VisitMut};

use crate::utilities::path_ext::PathExt;

pub struct VaryTypeExpander<'a> {
    variant: &'a Ident,
    pub errors: Vec<Error>,
}

impl<'a> VaryTypeExpander<'a> {
    /// Appends `variant` to the type in the first next expression.
    ///
    pub fn new(variant: &'a Ident) -> Self {
        Self {
            variant,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for VaryTypeExpander<'_> {
    fn visit_expr_struct_mut(&mut self, node: &mut ExprStruct) {
        self.visit_path_mut(&mut node.path);
    }

    fn visit_path_mut(&mut self, node: &mut Path) {
        *node = node.from_appendix(self.variant);
    }
}
