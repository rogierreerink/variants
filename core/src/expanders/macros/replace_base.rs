use syn::{Error, Ident, Type, visit_mut::VisitMut};

use crate::utilities::type_ext::TypePathExt;

use super::insert_type::InsertTypeMacro;

pub struct ReplaceBaseMacro<'a> {
    base_type: Type,
    variant: &'a Option<&'a Ident>,
    pub errors: Vec<Error>,
}

impl<'a> ReplaceBaseMacro<'a> {
    const IDENTIFIER: &'static str = "base";

    /// Replaces the `base!(type)` macro with the given type.
    ///
    pub fn new(base_type: Type, variant: &'a Option<&'a Ident>) -> Self {
        Self {
            base_type,
            variant,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for ReplaceBaseMacro<'_> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        let ty = self
            .variant
            .and_then(|variant| {
                if let Type::Path(type_path) = &self.base_type {
                    Some(Type::Path(type_path.clone().from_appendix(variant)))
                } else {
                    None
                }
            })
            .unwrap_or(self.base_type.clone());

        let mut insert_type_macro = InsertTypeMacro::new(Self::IDENTIFIER, ty);
        insert_type_macro.visit_type_mut(node);
        self.errors.append(&mut insert_type_macro.errors);
    }
}
