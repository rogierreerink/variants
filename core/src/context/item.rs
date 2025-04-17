use proc_macro2::Span;
use syn::{Error, Item, ItemEnum, ItemImpl, ItemStruct, visit_mut::VisitMut};

use super::{Context, r#enum::EnumContext, r#impl::ImplContext, r#struct::StructContext};

pub struct ItemContext<'a> {
    pub context: &'a Context,
    pub enum_ctx: Option<EnumContext<'a>>,
    pub impl_ctx: Option<ImplContext<'a>>,
    pub struct_ctx: Option<StructContext<'a>>,
    pub errors: Vec<Error>,
}

impl<'a> ItemContext<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self {
            context,
            enum_ctx: None,
            impl_ctx: None,
            struct_ctx: None,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for ItemContext<'_> {
    fn visit_item_mut(&mut self, node: &mut Item) {
        match node {
            Item::Enum(item) => self.visit_item_enum_mut(item),
            Item::Impl(item) => self.visit_item_impl_mut(item),
            Item::Struct(item) => self.visit_item_struct_mut(item),
            _ => self
                .errors
                .push(Error::new(Span::call_site(), "item not supported")),
        }
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        let mut enum_ctx = EnumContext::new(self.context);
        enum_ctx.visit_item_enum_mut(node);
        self.errors.append(&mut enum_ctx.errors);
        self.enum_ctx = Some(enum_ctx);
    }

    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        let mut impl_ctx = ImplContext::new(self.context);
        impl_ctx.visit_item_impl_mut(node);
        self.errors.append(&mut impl_ctx.errors);
        self.impl_ctx = Some(impl_ctx);
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let mut struct_ctx = StructContext::new(self.context);
        struct_ctx.visit_item_struct_mut(node);
        self.errors.append(&mut struct_ctx.errors);
        self.struct_ctx = Some(struct_ctx);
    }
}
