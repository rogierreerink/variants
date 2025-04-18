use proc_macro2::Span;
use syn::{Error, Item, ItemEnum, ItemImpl, ItemStruct, visit_mut::VisitMut};

use crate::{context::item::ItemContext, utilities::attribute_remover::AttributeRemover};

use super::{Context, r#enum::EnumExpander, r#impl::ImplExpander, r#struct::StructExpander};

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
            Item::Enum(item) => self.visit_item_enum_mut(item),
            Item::Impl(item) => self.visit_item_impl_mut(item),
            Item::Struct(item) => self.visit_item_struct_mut(item),
            _ => self
                .errors
                .push(Error::new(Span::call_site(), "item not supported")),
        }

        // Variant attributes are removed only at the very last step, as attributes play a role in
        // matching certain contexts to nodes. For example, field value contexts are stored in a
        // HashMap, with the field node hash as the key. If multiple field values with the same name,
        // type, visibility, etc exist anywhere in the item, they still may be having different
        // attributes, which determine how they must be processed. Keeping the attributes up until
        // all processing has been done solves this issue. If identical at different locations in
        // the item also share the same attributes, they might as well be stored under the same key.
        //
        // I would rather be able to uniquely identify nodes in more definitive way, such as with
        // span info, like line and column number or byte range, however: 1) this info is not stable
        // just yet, and more significantly, 2) this doesn't work in tests using `quote` to generate
        // input, as `quote` doesn't provide such information.
        AttributeRemover::new().visit_item_mut(node);
    }

    fn visit_item_enum_mut(&mut self, node: &mut ItemEnum) {
        let enum_ctx = match &self.item.enum_ctx {
            Some(context) => context,
            None => {
                self.errors.push(Error::new(
                    Span::call_site(),
                    "bug: enum context should exist",
                ));
                return;
            }
        };

        let mut enum_expander = EnumExpander::new(&mut self.context, &enum_ctx);
        enum_expander.visit_item_enum_mut(node);
        self.errors.append(&mut enum_expander.errors);
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
