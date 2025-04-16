use syn::{
    Ident, Result, Token,
    parse::{Parse, ParseStream},
};

pub mod field;
pub mod r#impl;
pub mod item;
pub mod r#struct;

pub struct Context {
    pub variants: Vec<Ident>,
}

impl Parse for Context {
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
