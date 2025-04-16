pub mod context;
pub mod errors;
pub mod expanders;
pub mod utilities;

#[cfg(test)]
mod tests {
    use crate::{
        context::{Context, item::ItemContext},
        errors::ErrorsExt,
        expanders::{self, item::ItemExpander},
    };

    use quote::quote;
    use syn::{Item, parse2, visit_mut::VisitMut};

    #[test]
    #[ignore = "not right now"]
    fn generate_structs() {
        let attrs = quote! {
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

        // prepare
        let mut item = parse2::<Item>(input).expect("failed to parse item");
        let ctx = parse2::<Context>(attrs).expect("failed to parse attributes");
        let mut item_ctx = ItemContext::new(&ctx);
        item_ctx.visit_item_mut(&mut item);

        println!(
            "{}",
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![item.clone()]
            })
        );

        if let Some(error) = item_ctx.errors.combine() {
            println!("{:#?}\n", error);
        }

        // expand base
        let mut expanded_item = item.clone();
        let expansion_ctx = expanders::Context::new(None);
        let mut item_expander = ItemExpander::new(&expansion_ctx, &item_ctx);
        item_expander.visit_item_mut(&mut expanded_item);

        println!(
            "{}",
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![expanded_item]
            })
        );

        if let Some(error) = item_expander.errors.combine() {
            println!("{:#?}\n", error);
        }

        // expand variants
        for variant in &ctx.variants {
            let mut expanded_item = item.clone();
            let expansion_ctx = expanders::Context::new(Some(variant));
            let mut item_expander = ItemExpander::new(&expansion_ctx, &item_ctx);
            item_expander.visit_item_mut(&mut expanded_item);

            println!(
                "{}",
                prettyplease::unparse(&syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![expanded_item]
                })
            );

            if let Some(error) = item_expander.errors.combine() {
                println!("{:#?}\n", error);
            }
        }
    }

    #[test]
    // #[ignore = "not right now"]
    fn generate_impls() {
        let attrs = quote! {
            Bar, Baz
        };

        let input = quote! {
            impl Foo {
                const NAME: &'static str = type_str!();

                fn new() -> Self {
                    let _ = "Expression in all variants";
                    let _ = type_str!();

                    match variant_str!() {
                        "Bar" | "Baz" => {
                            let _ = "Block only in FooBar and FooBaz";
                        }
                        _ => {
                            let _ = "Block not in FooBar or FooBaz";
                        }
                    }

                    if variant_str!() == "Bar" {
                        let _ = "Expression only in FooBar";
                    }

                    #[variants(include())]
                    let _ = "Expression only in Foo";

                    // Error: this macro should be in an expression
                    // variant_str!();

                    // The current variant should always be references with `Self`
                    Self::hi();

                    Self {
                        /// `bar` and `baz` will be included in all variants
                        bar: 123,
                        baz: 0.,

                        #[variants(exclude(Bar, Baz))]
                        bat: "¡hola, mundo!".into(),
                    }
                }

                fn hi() {
                    println("Hi, {}!", Self::NAME);
                }
            }
        };

        // prepare
        let mut item = parse2::<Item>(input).expect("failed to parse item");
        let ctx = parse2::<Context>(attrs).expect("failed to parse attributes");
        let mut item_ctx = ItemContext::new(&ctx);
        item_ctx.visit_item_mut(&mut item);

        println!(
            "{}",
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![item.clone()]
            })
        );

        if let Some(error) = item_ctx.errors.combine() {
            println!("{:#?}\n", error);
        }

        // expand base
        let mut expanded_item = item.clone();
        let expansion_ctx = expanders::Context::new(None);
        let mut item_expander = ItemExpander::new(&expansion_ctx, &item_ctx);
        item_expander.visit_item_mut(&mut expanded_item);

        println!(
            "{}",
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![expanded_item]
            })
        );

        if let Some(error) = item_expander.errors.combine() {
            println!("{:#?}\n", error);
        }

        // expand variants
        for variant in &ctx.variants {
            let mut expanded_item = item.clone();
            let expansion_ctx = expanders::Context::new(Some(variant));
            let mut item_expander = ItemExpander::new(&expansion_ctx, &item_ctx);
            item_expander.visit_item_mut(&mut expanded_item);

            println!(
                "{}",
                prettyplease::unparse(&syn::File {
                    shebang: None,
                    attrs: vec![],
                    items: vec![expanded_item]
                })
            );

            if let Some(error) = item_expander.errors.combine() {
                println!("{:#?}\n", error);
            }
        }
    }
}
