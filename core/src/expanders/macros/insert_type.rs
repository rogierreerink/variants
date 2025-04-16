use syn::{Error, Macro, Type, TypeMacro, TypePath, spanned::Spanned, visit_mut::VisitMut};

pub struct InsertTypeMacro<'a> {
    macro_name: &'static str,
    insert_type: Type,
    errors: &'a mut Vec<Error>,
}

impl<'a> InsertTypeMacro<'a> {
    /// Replace all type macros named `macro_name!()` with `insert_type`.
    ///
    pub fn new(macro_name: &'static str, insert_type: Type, errors: &'a mut Vec<Error>) -> Self {
        Self {
            macro_name,
            insert_type,
            errors,
        }
    }
}

impl VisitMut for InsertTypeMacro<'_> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        if let Type::Macro(TypeMacro { mac }) = node {
            if !mac.path.is_ident(self.macro_name) {
                return;
            }

            match &mut self.insert_type {
                Type::Path(TypePath { path, .. }) => {
                    for segment in &mut path.segments {
                        segment.ident.set_span(node.span());
                    }
                }
                _ => {
                    self.errors
                        .push(Error::new(node.span(), "cannot apply macro span to type"));
                }
            };

            *node = self.insert_type.clone();
        }
    }

    /// Emit an error on non-type macro invocations.
    ///
    fn visit_macro_mut(&mut self, node: &mut Macro) {
        if !node.path.is_ident(self.macro_name) {
            return;
        }

        self.errors.push(Error::new(
            node.span(),
            format!("`{}!()` may only be used in types", self.macro_name),
        ));
    }
}
