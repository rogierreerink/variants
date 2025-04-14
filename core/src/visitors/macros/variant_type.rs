use syn::{Error, Ident, Macro, Type, spanned::Spanned, visit_mut::VisitMut};

use crate::utilities::ident_ext::IdentExt;

use super::insert_type::InsertTypeMacro;

pub struct VariantTypeMacro<'a> {
    variant_type: Option<Type>,
    errors: &'a mut Vec<Error>,
}

impl<'a> VariantTypeMacro<'a> {
    const IDENTIFIER: &'static str = "variant_type";

    /// Replace all `variant_type!()` expression macros with a `Type` constructed
    /// out of `variant`.
    ///
    /// The macros span is preserved.
    ///
    pub fn new(variant: &Option<Ident>, errors: &'a mut Vec<Error>) -> Self {
        Self {
            variant_type: variant.as_ref().map(|variant| variant.clone().into_type()),
            errors,
        }
    }
}

impl VisitMut for VariantTypeMacro<'_> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        let variant_type = match &self.variant_type {
            Some(ty) => ty,
            None => {
                self.errors.push(Error::new(
                    node.span(),
                    format!("{}!() cannot be used in base types", Self::IDENTIFIER),
                ));
                return;
            }
        };

        InsertTypeMacro::new(Self::IDENTIFIER, variant_type.clone(), self.errors)
            .visit_type_mut(node);
    }

    /// Emit an error on non-type macro invocations.
    ///
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        let variant_type = match &self.variant_type {
            Some(ty) => ty,
            None => {
                self.errors.push(Error::new(
                    node.span(),
                    format!("{}!() cannot be used in base types", Self::IDENTIFIER),
                ));
                return;
            }
        };

        InsertTypeMacro::new(Self::IDENTIFIER, variant_type.clone(), self.errors)
            .visit_macro_mut(node);
    }
}
