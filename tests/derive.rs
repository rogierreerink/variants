#[cfg(test)]
mod tests {
    use variants::variants;

    #[test]
    fn derive_struct() {
        #[variants(Summary, Update)]
        #[allow(dead_code)]
        struct Foo {
            #[variants(include(Summary))]
            bar: usize,

            #[variants(include(Summary))]
            #[variants(include(Update), retype = "Option<{}>")]
            baz: String,

            #[variants(include(Update), retype = "Option<Box<{b}{v}>>")]
            ban: Option<Box<Foo>>,
        }

        let _ = Foo {
            bar: 0,
            baz: "hola, mundo".into(),
            ban: Some(Box::new(Foo {
                bar: 1,
                baz: "hello, world".into(),
                ban: None,
            })),
        };
        let _ = FooSummary {
            bar: 0,
            baz: "hola, mundo".into(),
        };
        let _ = FooUpdate {
            baz: Some("hola, mundo".into()),
            ban: Some(Box::new(FooUpdate {
                baz: Some("hello, world".into()),
                ban: None,
            })),
        };
    }

    #[test]
    fn derive_impl() {
        #[variants(Summary)]
        #[allow(dead_code)]
        struct Foo {
            #[variants(include(Summary))]
            id: usize,
            bar: String,
        }

        #[variants(Summary)]
        impl Foo {
            fn new() -> Self {
                Self {
                    #[variants(include(Summary))]
                    id: 0,
                    bar: "hola, mundo".into(),
                }
            }
        }

        let _ = Foo::new();
        let _ = FooSummary::new();
    }

    #[test]
    fn derive_impl_trait() {
        trait Hello {
            fn hello() -> String;
        }

        #[variants(Summary)]
        struct Foo;

        #[variants(Summary)]
        impl Hello for Foo {
            fn hello() -> String {
                let me: &str = type_str!();
                let msg;

                // The following if-else should get optimized away by the compiler,
                // leaving either one of the assignments depending on which impl is
                // being compiled.
                if variant_str!() == "Summary" {
                    msg = format!("Hola, {}", me);
                } else {
                    msg = format!("Hello, {}", me);
                }

                msg
            }
        }

        assert_eq!(&Foo::hello(), "Hello, Foo");
        assert_eq!(&FooSummary::hello(), "Hola, FooSummary");
    }

    #[test]
    fn derive_enum() {
        #[variants(Summary)]
        #[allow(dead_code)]
        enum Foo {
            Some {
                #[variants(include(Summary))]
                id: usize,
                name: String,
            },
            Tuple(
                u64,
                #[variants(include(Summary), retype = "Option<{}>")] String,
            ),
            Other,
        }

        let _ = Foo::Some {
            id: 0,
            name: "Bar".into(),
        };
        let _ = Foo::Tuple(0, "hi".into());
        let _ = Foo::Other;

        let _ = FooSummary::Some { id: 0 };
        let _ = FooSummary::Tuple(Some("hola".into()));
        let _ = FooSummary::Other;
    }

    #[test]
    fn derive_enum_and_struct() {
        #[variants(Summary)]
        #[allow(dead_code)]
        enum Foo {
            Some {
                #[variants(include(Summary))]
                id: usize,
                name: Boo,
            },
            Other {
                #[variants(include(Summary))]
                name: Boo,
            },
        }

        #[variants(Summary)]
        impl Foo {
            fn new() -> Self {
                Self::Some {
                    #[variants(include(Summary))]
                    id: 0,
                    name: Boo::new(),
                }
            }
        }

        #[allow(dead_code)]
        struct Boo {
            id: usize,
            name: String,
        }

        impl Boo {
            fn new() -> Self {
                Self {
                    id: 0,
                    name: "hi".into(),
                }
            }
        }

        let _: Foo = Foo::new();
        let _: FooSummary = FooSummary::new();
    }
}
