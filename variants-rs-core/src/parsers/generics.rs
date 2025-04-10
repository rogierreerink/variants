use std::ops::Index;

use syn::{
    Ident, Lit, Result, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::{self, Punctuated},
    token::Paren,
};

use super::TryParseExt;

#[derive(PartialEq, Debug)]
pub enum Value {
    Expr(Expression),
    Ident(Ident),
    List(List),
    Lit(Lit),
}

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        if let Ok(expr) = input.try_parse::<Expression>() {
            Ok(Self::Expr(expr))
        } else if let Ok(list) = input.try_parse::<List>() {
            Ok(Self::List(list))
        } else if let Ok(lit) = input.try_parse::<Lit>() {
            Ok(Self::Lit(lit))
        } else if let Ok(ident) = input.try_parse::<Ident>() {
            Ok(Self::Ident(ident))
        } else {
            Err(input.error("failed to parse attribute value: type of value cannot be determined"))
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    pub ident: Ident,
    pub eq_token: Token![=],
    pub value: Box<Value>,
}

impl Parse for Expression {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            ident: input.parse()?,
            eq_token: input.parse()?,
            value: input.parse()?,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct List {
    pub ident: Ident,
    pub paren_token: Paren,
    pub values: Punctuated<Value, Token![,]>,
}

impl Parse for List {
    fn parse(input: ParseStream) -> Result<Self> {
        let value_stream;

        Ok(Self {
            ident: input.parse()?,
            paren_token: parenthesized!(value_stream in input),
            values: value_stream.parse_terminated(Value::parse, Token![,])?,
        })
    }
}

impl Index<usize> for List {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl IntoIterator for List {
    type Item = Value;
    type IntoIter = punctuated::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use quote::quote;
    use syn::{Lit, parse2};

    use super::{Expression, List, Value};

    #[test]
    fn parse_expr() {
        let input = quote! {
            accept = true
        };

        match parse2::<Value>(input).unwrap() {
            Value::Expr(Expression { ident, value, .. }) => {
                assert_eq!(ident.to_string(), "accept");

                match value.as_ref() {
                    Value::Lit(Lit::Bool(lit_bool)) => {
                        assert_eq!(lit_bool.value(), true);
                    }
                    _ => panic!("input is not an ident"),
                }
            }
            _ => panic!("input is not an expression"),
        }
    }

    #[test]
    fn parse_ident() {
        let input = quote! {
            ident
        };

        match parse2::<Value>(input).unwrap() {
            Value::Ident(ident) => {
                assert_eq!(ident.to_string(), "ident")
            }
            _ => panic!("input is not an ident"),
        }
    }

    #[test]
    fn parse_list_of_string_literals() {
        let input = quote! {
            list("lit", "lit2")
        };

        match parse2::<Value>(input).unwrap() {
            Value::List(List { ident, values, .. }) => {
                assert_eq!(ident.to_string(), "list");

                match &values[0] {
                    Value::Lit(Lit::Str(lit_str)) => {
                        assert_eq!(lit_str.value(), "lit")
                    }
                    _ => panic!("list item 0 is not a string literal"),
                }

                match &values[1] {
                    Value::Lit(Lit::Str(lit_str)) => {
                        assert_eq!(lit_str.value(), "lit2")
                    }
                    _ => panic!("list item 1 is not a string literal"),
                }
            }
            _ => panic!("parsed value is not a list"),
        }
    }

    #[test]
    fn parse_list_of_idents() {
        let input = quote! {
            list(id, id2)
        };

        match parse2::<Value>(input).unwrap() {
            Value::List(List { ident, values, .. }) => {
                assert_eq!(ident.to_string(), "list");

                match &values[0] {
                    Value::Ident(ident) => {
                        assert_eq!(ident.to_string(), "id")
                    }
                    _ => panic!("list item 0 is not an ident"),
                }

                match &values[1] {
                    Value::Ident(ident) => {
                        assert_eq!(ident.to_string(), "id2")
                    }
                    _ => panic!("list item 1 is not an ident"),
                }
            }
            _ => panic!("parsed value is not a list"),
        }
    }

    #[test]
    fn parse_list_of_mixed_types() {
        let input = quote! {
            list(id, "lit", list2(123))
        };

        match parse2::<Value>(input).unwrap() {
            Value::List(List { ident, values, .. }) => {
                assert_eq!(ident.to_string(), "list");

                match &values[0] {
                    Value::Ident(ident) => {
                        assert_eq!(ident.to_string(), "id")
                    }
                    _ => panic!("list item 0 is not an ident"),
                }

                match &values[1] {
                    Value::Lit(Lit::Str(lit_str)) => {
                        assert_eq!(lit_str.value(), "lit")
                    }
                    _ => panic!("list item 1 is not a string literal"),
                }

                match &values[2] {
                    Value::List(List { ident, values, .. }) => {
                        assert_eq!(ident.to_string(), "list2");

                        match &values[0] {
                            Value::Lit(Lit::Int(lit_int)) => {
                                assert_eq!(lit_int.base10_digits(), "123")
                            }
                            _ => panic!("list2 item 0 is not an integer"),
                        }
                    }
                    _ => panic!("parsed value is not a list"),
                }
            }
            _ => panic!("parsed value is not a list"),
        }
    }
}
