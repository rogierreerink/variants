use syn::Ident;

pub mod expr_structs;
pub mod field;
pub mod field_value;
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
