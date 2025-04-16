use syn::{Error, ItemImpl, visit_mut::VisitMut};

pub struct ImplContext {
    pub errors: Vec<Error>,
}

impl ImplContext {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }
}

impl VisitMut for ImplContext {
    fn visit_item_impl_mut(&mut self, _node: &mut ItemImpl) {
        // println!("{:#?}\n", node);
    }
}
