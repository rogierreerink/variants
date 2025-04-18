use syn::{
    Error, Macro, Type, TypeMacro, TypePath,
    spanned::Spanned,
    visit_mut::{VisitMut, visit_type_mut},
};

pub struct InsertTypeMacro {
    macro_name: &'static str,
    insert_type: Type,
    pub errors: Vec<Error>,
}

impl InsertTypeMacro {
    /// Replace all type macros named `macro_name!()` with `insert_type`.
    ///
    pub fn new(macro_name: &'static str, insert_type: Type) -> Self {
        Self {
            macro_name,
            insert_type,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for InsertTypeMacro {
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
                    self.errors.push(Error::new(
                        node.span(),
                        "cannot apply macro span to replacement type",
                    ));
                }
            };

            *node = self.insert_type.clone();
        }

        visit_type_mut(self, node);
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
