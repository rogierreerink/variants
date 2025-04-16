use syn::{Error, Ident};

pub trait ErrorsExt {
    /// Combine all generated errors into a single error object. Each individual error will still be
    /// visible in the output stream.
    ///
    fn combine(&self) -> Option<Error>;

    /// Check of the existance of `variant` in `variants` and generate a standard error message
    /// when this is not the case. This functionality is used often throughout the library.
    ///
    fn contains_variant(&mut self, variants: &Vec<Ident>, variant: &Ident) -> bool;
}

impl ErrorsExt for Vec<Error> {
    fn combine(&self) -> Option<Error> {
        let first = match self.get(0) {
            Some(error) => error,
            None => return None,
        };

        let rest = match self.get(1..) {
            Some(errors) => errors,
            None => return None,
        };

        Some(rest.iter().fold(first.clone(), |mut error, next| {
            error.combine(next.clone());
            error
        }))
    }

    fn contains_variant(&mut self, variants: &Vec<Ident>, variant: &Ident) -> bool {
        let has_variant = variants.iter().any(|v| v == variant);

        if !variants.iter().any(|v| v == variant) {
            self.push(Error::new(
                variant.span(),
                format!("`{}` has not been declared as a variant", variant),
            ));
        }

        has_variant
    }
}
