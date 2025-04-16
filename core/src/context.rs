use syn::{
    Error, Ident, Path, Result, Token,
    parse::{Parse, ParseStream},
};

pub mod field;
pub mod r#impl;
pub mod item;
pub mod r#struct;

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

pub struct Context2 {
    pub variants: Vec<Ident>,
}

impl Parse for Context2 {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            variants: input
                .parse_terminated(Ident::parse, Token![,])?
                .iter()
                .map(|variant| variant.clone())
                .collect(),
        })
    }
}
