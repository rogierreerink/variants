pub mod item;
pub mod structure;

#[cfg(test)]
mod tests {
    use proc_macro2::Span;
    use quote::quote;
    use syn::Error;

    use crate::parsers::{
        generics::Values,
        helpers::{CombineErrorsExt, FromValueExt, IntoAttribute, IntoAttributeExt},
    };

    #[derive(PartialEq, Debug)]
    pub struct SomeAttribute {
        pub some_list: Vec<String>,
        pub some_bool: bool,
        pub some_expr: String,
    }

    impl IntoAttribute for SomeAttribute {
        fn try_from_spanned(values: Values, span: Span) -> syn::Result<Self> {
            let mut errors = Vec::new();

            let mut some_list: Option<Vec<String>> = None;
            let mut some_bool: Option<bool> = None;
            let mut some_expr: Option<String> = None;

            for value in values {
                let id = match value.identifier() {
                    Some(id) => id,
                    None => continue,
                };

                match id.as_str() {
                    id_str if id_str == "some_list" => {
                        some_list.from_value(id_str, value, &mut errors);
                    }
                    id_str if id_str == "some_bool" => {
                        some_bool.from_value(id_str, value, &mut errors);
                    }
                    id_str if id_str == "some_expr" => {
                        some_expr.from_value(id_str, value, &mut errors);
                    }
                    id_str => {
                        errors.push(Error::new(
                            value.span(),
                            format!("unrecognized entry `{}`", id_str),
                        ));
                    }
                }
            }

            if some_bool.is_none() {
                errors.push(Error::new(span, "expected key `some_expr` not found"));
            };
            if some_expr.is_none() {
                errors.push(Error::new(span, "expected key `some_expr` not found"));
            };

            if let Some(error) = errors.combine_errors() {
                return Err(error);
            }

            Ok(Self {
                some_list: some_list.unwrap_or_default(),
                some_bool: some_bool.unwrap_or_default(),
                some_expr: some_expr.unwrap_or_default(),
            })
        }
    }

    #[test]
    fn standard_types() {
        let input = quote! {
            some_list("lit1", "lit2"),
            some_bool,
            some_expr = "foo"
        };

        assert_eq!(
            input.into_attribute::<SomeAttribute>().unwrap(),
            SomeAttribute {
                some_list: vec!["lit1".into(), "lit2".into()],
                some_bool: true,
                some_expr: "foo".into(),
            }
        );
    }
}
