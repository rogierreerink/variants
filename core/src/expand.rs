#[cfg(test)]
use std::time::Instant;

#[cfg(test)]
use colored::Colorize;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, parse2, visit_mut::VisitMut};

use crate::{
    context::{self, item::ItemContext},
    expanders::{self, item::ItemExpander},
    utilities::errors_ext::ErrorsExt,
};

pub fn expand(attr: TokenStream, input: TokenStream) -> TokenStream {
    #[cfg(test)]
    let time_start = Instant::now();

    let mut item = match parse2::<Item>(input) {
        Ok(item) => item,
        Err(error) => return error.to_compile_error(),
    };

    let ctx = match parse2::<context::Context>(attr) {
        Ok(item) => item,
        Err(error) => return error.to_compile_error(),
    };

    let mut output = TokenStream::new();

    /* Create a context for the subsequent expanders to work with.
     */

    let mut item_ctx = ItemContext::new(&ctx);
    item_ctx.visit_item_mut(&mut item);

    if let Some(error) = item_ctx.errors.combine() {
        output.extend(error.into_compile_error());
    }

    /* Expand the base item.
     */

    let mut expanded_item = item.clone();
    let expansion_ctx = expanders::Context::new(None);
    let mut item_expander = ItemExpander::new(&expansion_ctx, &item_ctx);
    item_expander.visit_item_mut(&mut expanded_item);

    output.extend(quote! {
        #expanded_item
    });

    if let Some(error) = item_expander.errors.combine() {
        output.extend(error.into_compile_error());
    }

    /* Expand the variant items.
     */

    for variant in &ctx.variants {
        let mut expanded_item = item.clone();
        let expansion_ctx = expanders::Context::new(Some(variant));
        let mut item_expander = ItemExpander::new(&expansion_ctx, &item_ctx);
        item_expander.visit_item_mut(&mut expanded_item);

        output.extend(quote! {
            #[automatically_derived]
            #expanded_item
        });

        if let Some(error) = item_expander.errors.combine() {
            output.extend(error.into_compile_error());
        }
    }

    #[cfg(test)]
    {
        let time_end = Instant::now();
        let duration = time_end - time_start;
        println!(
            "{}",
            format!("{} duration: {}us", module_path!(), duration.as_micros()).yellow(),
        );
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::expand::expand;

    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn expand_structs() {
        let attr = quote! {
            Bar, Baz
        };

        let input = quote! {
            struct Foo {
                #[variants(include(Bar))]
                #[variants(include(Baz), retype = "Option<{}>")]
                bar: usize,

                #[variants(include(Baz), retype = "Option<{}>")]
                #[some(other = "stuff")]
                baz: f64,

                // `bat` will not be included in FooBar or FooBaz.
                bat: String,

                #[variants(include(Bar, Baz), retype = "Option<{b}{v}>")]
                recurse: Option<Foo>,
            }
        };

        let expect = quote! {
            struct Foo {
                bar: usize,
                #[some(other = "stuff")]
                baz: f64,
                bat: String,
                recurse: Option<Foo>,
            }
            #[automatically_derived]
            struct FooBar {
                bar: usize,
                recurse: Option<FooBar>,
            }
            #[automatically_derived]
            struct FooBaz {
                bar: Option<usize>,
                #[some(other = "stuff")]
                baz: Option<f64>,
                recurse: Option<FooBaz>,
            }
        };

        assert_eq_token_streams(&expand(attr, input), &expect);
    }

    #[test]
    fn expand_impls() {
        let attr = quote! {
            Bar
        };

        let input = quote! {
            impl Foo {
                const NAME: &'static str = type_str!();

                fn new() -> Self {
                    let _ = "Expression in all variants";
                    let _ = type_str!();

                    match variant_str!() {
                        "Bar" => {
                            let _ = "Block only in FooBar";
                        }
                        _ => {
                            let _ = "Block not in FooBar";
                        }
                    }

                    if variant_str!() == "Bar" {
                        let _ = "Expression only in FooBar";
                    }

                    #[variants(exclude(Bar))]
                    let _ = "Expression only in Foo";

                    // Error: this macro should be in an expression
                    // variant_str!();

                    // The current variant should always be references with `Self`
                    Self::hi();

                    Self {
                        #[variants(include(Bar))]
                        bar: 123,
                        #[variants(include(Bar))]
                        baz: 0.,
                        bat: "¡hola, mundo!".into(),
                    }
                }

                fn hi() {
                    println("Hi, {}!", Self::NAME);
                }
            }
        };

        let expect = quote! {
            impl Foo {
                const NAME: &'static str = "Foo";
                fn new() -> Self {
                    let _ = "Expression in all variants";
                    let _ = "Foo";
                    match "" {
                        "Bar" => {
                            let _ = "Block only in FooBar";
                        }
                        _ => {
                            let _ = "Block not in FooBar";
                        }
                    }
                    if "" == "Bar" {
                        let _ = "Expression only in FooBar";
                    }
                    let _ = "Expression only in Foo";
                    Self::hi();
                    Self {
                        bar: 123,
                        baz: 0.,
                        bat: "¡hola, mundo!".into(),
                    }
                }
                fn hi() {
                    println("Hi, {}!", Self::NAME);
                }
            }
            #[automatically_derived]
            impl FooBar {
                const NAME: &'static str = "FooBar";
                fn new() -> Self {
                    let _ = "Expression in all variants";
                    let _ = "FooBar";
                    match "Bar" {
                        "Bar" => {
                            let _ = "Block only in FooBar";
                        }
                        _ => {
                            let _ = "Block not in FooBar";
                        }
                    }
                    if "Bar" == "Bar" {
                        let _ = "Expression only in FooBar";
                    }
                    Self::hi();
                    Self {
                        bar: 123,
                        baz: 0.,
                    }
                }
                fn hi() {
                    println("Hi, {}!", Self::NAME);
                }
            }
        };

        assert_eq_token_streams(&expand(attr, input), &expect);
    }

    /// Pretty compare token streams for equality.
    ///
    pub fn assert_eq_token_streams(a: &TokenStream, b: &TokenStream) {
        let a_str = a.to_string();
        let a_parsed = syn::parse_file(&a_str).unwrap();
        let a_pretty = prettyplease::unparse(&a_parsed);

        let b_str = b.to_string();
        let b_parsed = syn::parse_file(&b_str).unwrap();
        let b_pretty = prettyplease::unparse(&b_parsed);

        pretty_assertions::assert_eq!(a_pretty, b_pretty);
    }
}
