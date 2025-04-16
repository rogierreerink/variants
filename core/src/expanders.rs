use syn::Ident;

pub mod field;
pub mod r#impl;
pub mod item;
pub mod macros;
pub mod r#struct;

pub struct Context<'a> {
    pub variant: Option<&'a Ident>,
}

impl<'a> Context<'a> {
    pub fn new(variant: Option<&'a Ident>) -> Self {
        Self { variant }
    }
}
