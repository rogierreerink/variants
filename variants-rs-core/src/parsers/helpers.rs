use proc_macro2::{Span, TokenStream};
use syn::{Error, parse::ParseStream};

use crate::parsers::generics::Value;

use super::generics::Values;

pub trait FromValueExt: Sized {
    fn from_value(&mut self, id: &str, value: Value, errors: &mut Vec<Error>);
}

impl<T> FromValueExt for Option<T>
where
    T: TryFrom<Value, Error = Error>,
{
    fn from_value(&mut self, id: &str, value: Value, errors: &mut Vec<Error>) {
        if self.is_some() {
            errors.push(Error::new(
                value.span(),
                format!("duplicate entry for `{}`", id),
            ));
        } else {
            match value.try_into() {
                Ok(value) => {
                    self.replace(value);
                }
                Err(error) => {
                    errors.push(error);
                }
            }
        }
    }
}

pub trait IntoAttribute: Sized {
    fn try_from_spanned(values: Values, span: Span) -> syn::Result<Self>;
}

pub trait IntoAttributeExt: Sized {
    fn into_attribute<T: IntoAttribute>(self) -> syn::Result<T>;
}

impl IntoAttributeExt for TokenStream {
    fn into_attribute<T: IntoAttribute>(self) -> syn::Result<T> {
        syn::parse::Parser::parse2(
            |input: ParseStream| T::try_from_spanned(input.parse()?, input.span()),
            self,
        )
    }
}

pub trait CombineErrorsExt: Sized {
    fn combine_errors(self) -> Option<Error>;
}

impl CombineErrorsExt for Vec<Error> {
    fn combine_errors(self) -> Option<Error> {
        let first = match self.get(0) {
            Some(first) => first.clone(),
            None => return None,
        };

        let rest = match self.get(1..) {
            Some(rest) => rest,
            None => return Some(first),
        };

        Some(rest.iter().fold(first, |mut acc, next| {
            acc.combine(next.clone());
            acc
        }))
    }
}
