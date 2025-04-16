use proc_macro2::Span;
use syn::{Error, Item, ItemImpl, ItemStruct, visit_mut::VisitMut};

use super::{Context, r#impl::ImplContext, r#struct::StructContext};

pub struct ItemContext<'a> {
    pub context: &'a Context,
    pub impl_ctx: Option<ImplContext>,
    pub struct_ctx: Option<StructContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> ItemContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            impl_ctx: None,
            struct_ctx: None,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for ItemContext<'_> {
    fn visit_item_mut(&mut self, node: &mut Item) {
        match node {
            Item::Impl(item) => self.visit_item_impl_mut(item),
            Item::Struct(item) => self.visit_item_struct_mut(item),
            _ => self
                .errors
                .push(Error::new(Span::call_site(), "item not supported")),
        }
    }

    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        let mut impl_ctx = ImplContext::new();
        impl_ctx.visit_item_impl_mut(node);
        self.errors.append(&mut impl_ctx.errors);
        self.impl_ctx = Some(impl_ctx);
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let mut struct_ctx = StructContext::new(&mut self.context);
        struct_ctx.visit_item_struct_mut(node);
        self.errors.append(&mut struct_ctx.errors);
        self.struct_ctx = Some(struct_ctx);
    }
}
