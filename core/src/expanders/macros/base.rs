use syn::{Error, Type, TypeMacro, parse2, visit_mut::VisitMut};

pub struct BaseMacro {
    pub base_type: Option<Type>,
    pub errors: Vec<Error>,
}

impl BaseMacro {
    const IDENTIFIER: &'static str = "base";

    /// Identifies the base variant to be implemented, using notation: `base!(type)`.
    /// The base type and variant identifier, if not `None`, are concatenated and replaces the
    /// macro.
    ///
    pub fn new() -> Self {
        Self {
            base_type: None,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for BaseMacro {
    fn visit_type_macro_mut(&mut self, node: &mut TypeMacro) {
        if !node.mac.path.is_ident(Self::IDENTIFIER) {
            return;
        }

        self.base_type = match parse2::<Type>(node.mac.tokens.clone()) {
            Ok(ty) => Some(ty),
            Err(error) => {
                self.errors.push(error);
                return;
            }
        };
    }
}
