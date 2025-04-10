use syn::Ident;

#[derive(Debug)]
pub struct ItemAttribute {
    pub variants: Vec<Ident>,
}
