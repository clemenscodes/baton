use ::syn::visit::{self, Visit};
use ::syn::{
    Attribute, ConstParam, ExprClosure, ExprForLoop, Field, FnArg, ForeignItemFn,
    ForeignItemStatic, ForeignItemType, ImplItem, ImplItemConst, ImplItemFn, ImplItemType,
    ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemImpl, ItemMacro, ItemMod, ItemStatic,
    ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, Label, Lifetime, LifetimeParam,
    Local, Pat, PatIdent, PatType, Signature, TraitItem, TraitItemConst, TraitItemFn,
    TraitItemType, TypeParam, UseRename, Variant,
};
use proc_macro2::{Ident, LineColumn};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct DeclarationVisitor {
    starts: BTreeSet<LineColumn>,
    test_function_starts: BTreeSet<LineColumn>,
}

impl Layer for DeclarationVisitor {
    type Layer = InfrastructureLayer;
}

impl DeclarationVisitor {
    const PROGRAM_START_NAME: &'static str = "main";
    const CONSTANT_CONSTRUCTOR_NAME: &'static str = "new";
    const TEST_NAMES: [&'static str; 2] = ["test", "wasm_bindgen_test"];

    pub(super) fn starts(&self) -> &BTreeSet<LineColumn> {
        match self {
            Self { starts, .. } => starts,
        }
    }

    pub(super) fn test_function_starts(&self) -> &BTreeSet<LineColumn> {
        &self.test_function_starts
    }

    fn insert_declaration(&mut self, identifier: &Ident) {
        let Self { starts, .. } = self;
        let span = identifier.span();
        let start_position = span.start();
        starts.insert(start_position);
    }

    fn insert_test_function(&mut self, identifier: &Ident) {
        let Self {
            test_function_starts,
            ..
        } = self;
        let span = identifier.span();
        let start_position = span.start();
        test_function_starts.insert(start_position);
    }

    fn insert_signature(&mut self, signature: &Signature) {
        let Signature {
            constness: constant_marker,
            ident: identifier,
            ..
        } = signature;
        if constant_marker.is_none() || identifier != Self::CONSTANT_CONSTRUCTOR_NAME {
            self.insert_declaration(identifier);
        }
    }

    fn insert_binding(&mut self, pattern: &Pat) {
        match pattern {
            Pat::Ident(PatIdent {
                ident: identifier, ..
            }) => self.insert_declaration(identifier),
            Pat::Type(PatType {
                pat: typed_pattern, ..
            }) => self.insert_binding(typed_pattern),
            _ => {}
        }
    }

    fn insert_implementation_item(&mut self, implementation_item: &ImplItem) {
        match implementation_item {
            ImplItem::Fn(ImplItemFn { sig: signature, .. }) => self.insert_signature(signature),
            ImplItem::Const(ImplItemConst {
                ident: identifier, ..
            }) => self.insert_declaration(identifier),
            ImplItem::Type(ImplItemType {
                ident: identifier, ..
            }) => self.insert_declaration(identifier),
            _ => {}
        }
    }

    fn insert_trait_item(&mut self, trait_item: &TraitItem) {
        match trait_item {
            TraitItem::Fn(TraitItemFn { sig: signature, .. }) => self.insert_signature(signature),
            TraitItem::Const(TraitItemConst {
                ident: identifier, ..
            }) => self.insert_declaration(identifier),
            TraitItem::Type(TraitItemType {
                ident: identifier, ..
            }) => self.insert_declaration(identifier),
            _ => {}
        }
    }
}

impl<'syntax> Visit<'syntax> for DeclarationVisitor {
    fn visit_item_fn(&mut self, node: &'syntax ItemFn) {
        let ItemFn {
            attrs: attributes,
            sig: Signature {
                ident: identifier, ..
            },
            ..
        } = node;
        let test_names = Self::TEST_NAMES;
        let test_attribute_criterion = |attribute: &&Attribute| {
            let attribute_path = attribute.path();
            let test_name_criterion = |test_name: &&str| attribute_path.is_ident(test_name);
            test_names.iter().any(test_name_criterion)
        };
        let test_attribute_count = attributes.iter().filter(test_attribute_criterion).count();
        if test_attribute_count > 0 {
            self.insert_test_function(identifier);
        } else if identifier != Self::PROGRAM_START_NAME {
            self.insert_declaration(identifier);
        }
        visit::visit_item_fn(self, node);
    }

    fn visit_item_impl(&mut self, node: &'syntax ItemImpl) {
        if let ItemImpl {
            trait_: None,
            items: implementation_items,
            ..
        } = node
        {
            for implementation_item in implementation_items {
                self.insert_implementation_item(implementation_item);
            }
        }
        visit::visit_item_impl(self, node);
    }

    fn visit_item_trait(&mut self, node: &'syntax ItemTrait) {
        let ItemTrait {
            ident: identifier,
            items: trait_items,
            ..
        } = node;
        self.insert_declaration(identifier);
        for trait_item in trait_items {
            self.insert_trait_item(trait_item);
        }
        visit::visit_item_trait(self, node);
    }

    fn visit_item_struct(&mut self, node: &'syntax ItemStruct) {
        let ItemStruct {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_struct(self, node);
    }

    fn visit_item_enum(&mut self, node: &'syntax ItemEnum) {
        let ItemEnum {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_enum(self, node);
    }

    fn visit_item_union(&mut self, node: &'syntax ItemUnion) {
        let ItemUnion {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_union(self, node);
    }

    fn visit_item_type(&mut self, node: &'syntax ItemType) {
        let ItemType {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_type(self, node);
    }

    fn visit_item_const(&mut self, node: &'syntax ItemConst) {
        let ItemConst {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_const(self, node);
    }

    fn visit_item_static(&mut self, node: &'syntax ItemStatic) {
        let ItemStatic {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_static(self, node);
    }

    fn visit_item_mod(&mut self, node: &'syntax ItemMod) {
        let ItemMod {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_mod(self, node);
    }

    fn visit_item_trait_alias(&mut self, node: &'syntax ItemTraitAlias) {
        let ItemTraitAlias {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_item_trait_alias(self, node);
    }

    fn visit_item_macro(&mut self, node: &'syntax ItemMacro) {
        if let ItemMacro {
            ident: Some(identifier),
            ..
        } = node
        {
            self.insert_declaration(identifier);
        }
        visit::visit_item_macro(self, node);
    }

    fn visit_item_extern_crate(&mut self, node: &'syntax ItemExternCrate) {
        if let ItemExternCrate {
            rename: Some((_, identifier)),
            ..
        } = node
        {
            self.insert_declaration(identifier);
        }
        visit::visit_item_extern_crate(self, node);
    }

    fn visit_foreign_item_fn(&mut self, node: &'syntax ForeignItemFn) {
        let ForeignItemFn { sig: signature, .. } = node;
        self.insert_signature(signature);
        visit::visit_foreign_item_fn(self, node);
    }

    fn visit_foreign_item_static(&mut self, node: &'syntax ForeignItemStatic) {
        let ForeignItemStatic {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_foreign_item_static(self, node);
    }

    fn visit_foreign_item_type(&mut self, node: &'syntax ForeignItemType) {
        let ForeignItemType {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_foreign_item_type(self, node);
    }

    fn visit_field(&mut self, node: &'syntax Field) {
        if let Field {
            ident: Some(identifier),
            ..
        } = node
        {
            self.insert_declaration(identifier);
        }
        visit::visit_field(self, node);
    }

    fn visit_variant(&mut self, node: &'syntax Variant) {
        let Variant {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_variant(self, node);
    }

    fn visit_type_param(&mut self, node: &'syntax TypeParam) {
        let TypeParam {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_type_param(self, node);
    }

    fn visit_const_param(&mut self, node: &'syntax ConstParam) {
        let ConstParam {
            ident: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_const_param(self, node);
    }

    fn visit_lifetime_param(&mut self, node: &'syntax LifetimeParam) {
        let LifetimeParam {
            lifetime: Lifetime {
                ident: identifier, ..
            },
            ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_lifetime_param(self, node);
    }

    fn visit_label(&mut self, node: &'syntax Label) {
        let Label {
            name: Lifetime {
                ident: identifier, ..
            },
            ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_label(self, node);
    }

    fn visit_use_rename(&mut self, node: &'syntax UseRename) {
        let UseRename {
            rename: identifier, ..
        } = node;
        self.insert_declaration(identifier);
        visit::visit_use_rename(self, node);
    }

    fn visit_local(&mut self, node: &'syntax Local) {
        let Local { pat: pattern, .. } = node;
        self.insert_binding(pattern);
        visit::visit_local(self, node);
    }

    fn visit_fn_arg(&mut self, node: &'syntax FnArg) {
        if let FnArg::Typed(PatType { pat: pattern, .. }) = node {
            self.insert_binding(pattern);
        }
        visit::visit_fn_arg(self, node);
    }

    fn visit_expr_closure(&mut self, node: &'syntax ExprClosure) {
        let ExprClosure {
            inputs: patterns, ..
        } = node;
        for pattern in patterns {
            self.insert_binding(pattern);
        }
        visit::visit_expr_closure(self, node);
    }

    fn visit_expr_for_loop(&mut self, node: &'syntax ExprForLoop) {
        let ExprForLoop { pat: pattern, .. } = node;
        self.insert_binding(pattern);
        visit::visit_expr_for_loop(self, node);
    }

    fn visit_pat_ident(&mut self, node: &'syntax PatIdent) {
        let PatIdent {
            by_ref: reference_marker,
            mutability: mutability_marker,
            ident: identifier,
            subpat: subpattern,
            ..
        } = node;
        let identifier_text = identifier.to_string();
        if reference_marker.is_some()
            || mutability_marker.is_some()
            || subpattern.is_some()
            || !identifier_text.starts_with(char::is_uppercase)
        {
            self.insert_declaration(identifier);
        }
        visit::visit_pat_ident(self, node);
    }
}
