use syn::{Field, Fields};

use super::vec_ext::VecExt;

pub trait FieldsExt {
    fn replace_fields(&mut self, fields: Vec<Field>);
}

impl FieldsExt for Fields {
    fn replace_fields(&mut self, fields: Vec<Field>) {
        match self {
            Fields::Named(f) => {
                f.named = fields.into_punctuated();
            }
            Fields::Unnamed(f) => {
                f.unnamed = fields.into_punctuated();
            }
            _ => {}
        }
    }
}
