use proc_macro2::Span;
use syn::{Error, Item, ItemImpl, ItemStruct, visit_mut::VisitMut};

use crate::context::item::ItemContext;

use super::{Context, r#impl::ImplExpander, r#struct::StructExpander};

pub struct ItemExpander<'a> {
    context: &'a Context<'a>,
    item: &'a ItemContext<'a>,
    pub errors: Vec<Error>,
}

impl<'a> ItemExpander<'a> {
    pub fn new(context: &'a Context, item: &'a ItemContext) -> Self {
        Self {
            context,
            item,
            errors: Default::default(),
        }
    }
}

impl VisitMut for ItemExpander<'_> {
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
        let impl_ctx = match &self.item.impl_ctx {
            Some(context) => context,
            None => {
                self.errors.push(Error::new(
                    Span::call_site(),
                    "bug: impl context should exist",
                ));
                return;
            }
        };

        let mut impl_expander = ImplExpander::new(&mut self.context, &impl_ctx);
        impl_expander.visit_item_impl_mut(node);
        self.errors.append(&mut impl_expander.errors);
    }

    fn visit_item_struct_mut(&mut self, node: &mut ItemStruct) {
        let struct_ctx = match &self.item.struct_ctx {
            Some(context) => context,
            None => {
                self.errors.push(Error::new(
                    Span::call_site(),
                    "bug: struct context should exist",
                ));
                return;
            }
        };

        let mut struct_expander = StructExpander::new(&mut self.context, &struct_ctx);
        struct_expander.visit_item_struct_mut(node);
        self.errors.append(&mut struct_expander.errors);
    }
}
