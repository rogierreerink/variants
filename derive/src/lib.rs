use proc_macro::TokenStream;
use variants_core::expand::expand;

#[proc_macro_attribute]
pub fn variants(attr: TokenStream, input: TokenStream) -> TokenStream {
    expand(attr.into(), input.into()).into()
}
