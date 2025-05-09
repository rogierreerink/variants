use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, parse2, visit_mut::VisitMut};

use crate::{
    context::{self, item::ItemContext},
    expanders::{self, item::ItemExpander},
    utilities::errors_ext::ErrorsExt,
};

pub fn expand(attr: TokenStream, input: TokenStream) -> TokenStream {
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

    output
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::expand::expand;

    use colored::Colorize;
    use proc_macro2::TokenStream;
    use quote::quote;

    #[test]
    fn expand_named_structs() {
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

        let time_start = Instant::now();
        let expanded = expand(attr, input);
        let time_end = Instant::now();

        assert_eq_token_streams(&expanded, &expect);
        assess_expansion_duration(time_start, time_end, 2000);
    }

    #[test]
    fn expand_unnamed_structs() {
        let attr = quote! {
            Bar, Baz
        };

        let input = quote! {
            struct Foo (
                #[variants(include(Bar))]
                #[variants(include(Baz), retype = "Option<{}>")]
                usize,

                #[variants(include(Baz), retype = "Option<{}>")]
                #[some(other = "stuff")]
                f64,

                // `bat` will not be included in FooBar or FooBaz.
                String,

                #[variants(include(Bar, Baz), retype = "Option<{b}{v}>")]
                Option<Foo>,
            );
        };

        let expect = quote! {
            struct Foo (
                usize,
                #[some(other = "stuff")]
                f64,
                String,
                Option<Foo>,
            );
            #[automatically_derived]
            struct FooBar (
                usize,
                Option<FooBar>,
            );
            #[automatically_derived]
            struct FooBaz (
                Option<usize>,
                #[some(other = "stuff")]
                Option<f64>,
                Option<FooBaz>,
            );
        };

        let time_start = Instant::now();
        let expanded = expand(attr, input);
        let time_end = Instant::now();

        assert_eq_token_streams(&expanded, &expect);
        assess_expansion_duration(time_start, time_end, 2000);
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

        let time_start = Instant::now();
        let expanded = expand(attr, input);
        let time_end = Instant::now();

        assert_eq_token_streams(&expanded, &expect);
        assess_expansion_duration(time_start, time_end, 2000);
    }

    #[test]
    fn expand_enum() {
        let attr = quote! {
            Bar, Baz
        };

        let input = quote! {
            enum Foo {
                Struct {
                    #[variants(include(Bar, Baz))]
                    id: usize,

                    #[variants(include(Bar), retype = "Box<{t}>")]
                    name: String,
                },
                Tuple (
                    #[variants(include(Bar))]
                    u64,

                    #[variants(include(Baz), retype = "Option<{t}>")]
                    String
                ),
                Empty,
            }
        };

        let expect = quote! {
            enum Foo {
                Struct {
                    id: usize,
                    name: String,
                },
                Tuple (
                    u64,
                    String
                ),
                Empty,
            }
            #[automatically_derived]
            enum FooBar {
                Struct {
                    id: usize,
                    name: Box<String>,
                },
                Tuple (
                    u64
                ),
                Empty,
            }
            #[automatically_derived]
            enum FooBaz {
                Struct {
                    id: usize,
                },
                Tuple (
                    Option<String>
                ),
                Empty,
            }
        };

        let time_start = Instant::now();
        let expanded = expand(attr, input);
        let time_end = Instant::now();

        assert_eq_token_streams(&expanded, &expect);
        assess_expansion_duration(time_start, time_end, 2000);
    }

    fn assert_eq_token_streams(a: &TokenStream, b: &TokenStream) {
        let a_str = a.to_string();
        let a_parsed = syn::parse_file(&a_str).unwrap();
        let a_pretty = prettyplease::unparse(&a_parsed);

        let b_str = b.to_string();
        let b_parsed = syn::parse_file(&b_str).unwrap();
        let b_pretty = prettyplease::unparse(&b_parsed);

        pretty_assertions::assert_eq!(a_pretty, b_pretty);
    }

    fn assess_expansion_duration(start: Instant, end: Instant, lt_us: u128) {
        let duration = (end - start).as_micros();
        let duration_str = format!("expansion duration: {}us", duration);
        if duration >= lt_us {
            println!("{}", duration_str.red(),);
        } else {
            println!("{}", duration_str.yellow(),);
        }
    }
}
