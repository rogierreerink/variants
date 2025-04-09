pub mod context;
pub mod generator;
pub mod utilities;
pub mod visitors;

#[cfg(test)]
mod tests {
    use crate::visitors::items::item::ItemVisitor;

    use proc_macro2::Span;
    use quote::quote;
    use syn::{Ident, Item, parse2, visit_mut::VisitMut};

    #[test]
    // #[ignore = "working on impl right now"]
    fn generate_structs() {
        let input = quote! {
            #[variants(Test)]
            struct Foo {
                #[variants(include(Test))]
                bar: usize,

                #[variants(include(Test), retype = "Option<{}>")]
                baz: f64,

                // `bat` will not be included in FooTest.
                /// This doc-comment and other non-`variants` attributes will.
                bat: String,

                // Foo will automatically be renamed into its variants.
                recurse: Foo,
                recurse_vec: Vec<Foo>,
            }
        };

        // let expected = quote! {
        //     struct Foo {
        //         bar: usize,
        //         baz: f64,
        //         /// This doc-comment and other non-`variants` attributes will.
        //         bat: String,
        //     }
        //     struct FooTest {
        //         bar: usize,
        //         baz: Option<f64>,
        //     }
        // };

        let mut input_ast = parse2::<Item>(input).expect("input must be parsable by syn");
        let mut errors = Vec::new();
        let variant = Some(Ident::new("Bar".into(), Span::call_site()));

        ItemVisitor::new(&variant, &mut errors).visit_item_mut(&mut input_ast);

        println!(
            "{}",
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![input_ast]
            })
        );

        println!("{:#?}", errors);
    }

    #[test]
    #[ignore = "working on struct right now"]
    fn generate_impls() {
        let input = quote! {
            #[variants(Bar, Baz)]
            impl Foo {
                const NAME: &'static str = type_str!();

                fn new() -> Self {
                    let _ = "Expression in all variants";
                    let _: variant_type!() = Bar {};

                    match variant_str!() {
                        "Bar" | "Baz" => {
                            let _ = "Block only in FooBar and FooBaz";
                        }
                        _ => {
                            let _ = "Block not in FooBar nor FooBaz";
                        }
                    }

                    if variant_str!() == "Bar" {
                        let _ = "Block only in FooBar";
                    }

                    #[variants(include())]
                    let _ = "Expression only in Foo";

                    // This macro should be in an expression
                    variant_str!();

                    Foo::hi();

                    Self {
                        /// `bar` and `baz` will be included in all variants
                        bar: 123,
                        baz: 0.,

                        #[variants(exclude(Bar, Baz))]
                        bat: "¡hola, mundo!".into(),
                    }
                }

                fn hi() {
                    println("Hi, {}!", Foo::NAME);
                }
            }
        };

        let mut input_ast = parse2::<Item>(input).expect("input must be parsable by syn");
        let mut errors = Vec::new();
        let variant = Some(Ident::new("Bar".into(), Span::call_site()));

        ItemVisitor::new(&variant, &mut errors).visit_item_mut(&mut input_ast);

        println!(
            "{}",
            prettyplease::unparse(&syn::File {
                shebang: None,
                attrs: vec![],
                items: vec![input_ast]
            })
        );

        println!("{:#?}", errors);
    }
}
