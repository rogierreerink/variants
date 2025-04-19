use proc_macro2::Span;
use syn::{Error, ItemImpl, Type, visit_mut::VisitMut};

use crate::{context::r#impl::ImplContext, utilities::type_ext::TypePathExt};

use super::{
    Context,
    block::BlockExpander,
    expr_structs::ExprStructExpander,
    macros::{
        base::BaseMacro, replace_base::ReplaceBaseMacro, ty::TyMacro, type_str::TypeStrMacro,
        variant_str::VariantStrMacro,
    },
};

pub struct ImplExpander<'a> {
    context: &'a Context<'a>,
    impl_ctx: &'a ImplContext<'a>,
    pub errors: Vec<Error>,
}

impl<'a> ImplExpander<'a> {
    pub fn new(context: &'a Context, impl_ctx: &'a ImplContext) -> Self {
        Self {
            context,
            impl_ctx,
            errors: Vec::new(),
        }
    }
}

impl VisitMut for ImplExpander<'_> {
    fn visit_item_impl_mut(&mut self, node: &mut ItemImpl) {
        let mut base_macro = BaseMacro::new();
        base_macro.visit_type_mut(&mut node.self_ty);
        self.errors.append(&mut base_macro.errors);

        let base_ty = base_macro
            .base_type
            .clone()
            .unwrap_or(node.self_ty.as_ref().clone());

        let ty_path = match &base_ty {
            Type::Path(ty_path) => ty_path,
            _ => {
                self.errors.push(Error::new(
                    Span::call_site(),
                    "implemented type must be a type path",
                ));
                return;
            }
        };

        let mut type_str_macro = TypeStrMacro::new(&ty_path, &self.context.variant);
        type_str_macro.visit_item_impl_mut(node);
        self.errors.append(&mut type_str_macro.errors);

        let mut variant_str_macro = VariantStrMacro::new(&self.context.variant);
        variant_str_macro.visit_item_impl_mut(node);
        self.errors.append(&mut variant_str_macro.errors);

        let mut type_macro = TyMacro::new(&base_ty, &self.context.variant);
        type_macro.visit_item_impl_mut(node);
        self.errors.append(&mut type_macro.errors);

        let mut block_expander = BlockExpander::new(self.context, &self.impl_ctx.stmt_ctxs);
        block_expander.visit_item_impl_mut(node);
        self.errors.append(&mut block_expander.errors);

        let mut expr_struct_expander =
            ExprStructExpander::new(self.context, &self.impl_ctx.field_value_ctxs);
        expr_struct_expander.visit_item_impl_mut(node);
        self.errors.append(&mut expr_struct_expander.errors);

        if let Some(base_ty) = &base_macro.base_type {
            let mut replace_base_macro = ReplaceBaseMacro::new(base_ty, &self.context.variant);
            replace_base_macro.visit_type_mut(&mut node.self_ty);
            self.errors.append(&mut replace_base_macro.errors);
        } else if let Some(variant) = self.context.variant {
            node.self_ty = Box::new(ty_path.from_appendix(variant).into_type());
        }
    }
}
