use syn::{Error, Ident, Type, visit_mut::VisitMut};

use crate::utilities::type_ext::TypePathExt;

use super::insert_type::InsertTypeMacro;

pub struct TyMacro<'a> {
    base_type: &'a Type,
    variant: &'a Option<&'a Ident>,
    pub errors: Vec<Error>,
}

impl<'a> TyMacro<'a> {
    const IDENTIFIER: &'static str = "ty";

    /// Replaces the `ty!()` macro with a concatenation of the base type and the
    /// variant. If the variant is `None`, just the base type is inserted.
    ///
    pub fn new(base_type: &'a Type, variant: &'a Option<&'a Ident>) -> Self {
        Self {
            base_type,
            variant,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for TyMacro<'_> {
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
