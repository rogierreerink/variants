use syn::{Attribute, visit_mut::VisitMut};

pub struct AttributeRemover {}

impl AttributeRemover {
    pub fn new() -> Self {
        Self {}
    }
}

impl VisitMut for AttributeRemover {
    fn visit_attributes_mut(&mut self, node: &mut Vec<Attribute>) {
        node.retain(|attr| !attr.path().is_ident("variants"));
    }
}
