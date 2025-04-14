use syn::{Error, Ident, Path};

pub struct Context {
    pub base_path: Option<Path>,
    pub base: Ident,
    pub variant: Option<Ident>,
    pub errors: Vec<Error>,
}

impl Context {
    pub fn new(base: Ident, variant: Option<Ident>) -> Self {
        Self {
            base_path: None,
            base,
            variant,
            errors: Vec::new(),
        }
    }
}
