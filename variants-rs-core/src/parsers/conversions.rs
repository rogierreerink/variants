use syn::{Error, Lit, Result};

use super::{
    generics::{Expr, List, Value},
    helpers::CombineErrorsExt,
};

impl TryFrom<Value> for bool {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Expr(Expr { value, .. }) => match value.as_ref() {
                Value::Lit(Lit::Bool(lit_bool)) => Ok(lit_bool.value()),
                value => Err(Error::new(value.span(), "expected a boolean expression")),
            },
            Value::Ident(_) => Ok(true),
            value => Err(Error::new(
                value.span(),
                match value.identifier() {
                    Some(id) => format!("`{}` must be a boolean expression", id),
                    None => "expected a boolean expression".into(),
                },
            )),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::Expr(Expr { value, .. }) => match value.as_ref() {
                Value::Lit(Lit::Str(lit_str)) => Ok(lit_str.value()),
                value => Err(Error::new(
                    value.span(),
                    "expected an identifier or string expression",
                )),
            },
            Value::Lit(Lit::Str(lit_str)) => Ok(lit_str.value()),
            value => Err(Error::new(
                value.span(),
                match value.identifier() {
                    Some(id) => format!("`{}` must be a string literal", id),
                    None => "expected a string literal".into(),
                },
            )),
        }
    }
}

impl TryFrom<Value> for Vec<String> {
    type Error = Error;

    fn try_from(value: Value) -> Result<Self> {
        match value {
            Value::List(List { values, .. }) => {
                let mut errors = vec![];
                let mut strings = vec![];

                for value in values {
                    match value {
                        Value::Lit(Lit::Str(lit_str)) => strings.push(lit_str.value()),
                        value => errors.push(Error::new(
                            value.span(),
                            match value.identifier() {
                                Some(id) => format!("`{}` must be a string literal", id),
                                None => "expected a string literal".into(),
                            },
                        )),
                    }
                }

                if let Some(error) = errors.combine_errors() {
                    return Err(error);
                }

                Ok(strings)
            }
            value => Err(Error::new(
                value.span(),
                match value.identifier() {
                    Some(id) => format!("`{}` must contain a list of strings", id),
                    None => "expected a list of strings".into(),
                },
            )),
        }
    }
}
