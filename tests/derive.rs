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
}
