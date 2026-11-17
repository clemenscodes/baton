use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Syntax;

impl Layer for Syntax {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Syntax {}

impl Syntax {
    const NAME: &'static str = "syn";

    const NAMES: &'static [&'static str] = &[
        "Attribute",
        "attrs",
        "by_ref",
        "constness",
        "ConstParam",
        "content",
        "ExprClosure",
        "ExprForLoop",
        "ext",
        "Field",
        "File",
        "FnArg",
        "ForeignItemFn",
        "ForeignItemStatic",
        "ForeignItemType",
        "ident",
        "IdentExt",
        "ImplItem",
        "ImplItemConst",
        "ImplItemFn",
        "ImplItemType",
        "inputs",
        "is_ident",
        "Item",
        "ItemConst",
        "ItemEnum",
        "ItemExternCrate",
        "ItemFn",
        "ItemImpl",
        "ItemMacro",
        "ItemMod",
        "items",
        "ItemStatic",
        "ItemStruct",
        "ItemTrait",
        "ItemTraitAlias",
        "ItemType",
        "ItemUnion",
        "Label",
        "Lifetime",
        "lifetime",
        "LifetimeParam",
        "List",
        "Lit",
        "Local",
        "Meta",
        "meta",
        "MetaList",
        "Mod",
        "mutability",
        "name",
        "NameValue",
        "parse2",
        "Pat",
        "pat",
        "path",
        "PatIdent",
        "PatType",
        "pound_token",
        "rename",
        "sig",
        "Signature",
        "spans",
        "subpat",
        "suffix",
        "syn",
        "tokens",
        "trait_",
        "TraitItem",
        "TraitItemConst",
        "TraitItemFn",
        "TraitItemType",
        "Typed",
        "TypeParam",
        "unraw",
        "UseRename",
        "Variant",
        "Visit",
        "visit",
        "visit_attribute",
        "visit_const_param",
        "visit_expr_closure",
        "visit_expr_for_loop",
        "visit_field",
        "visit_file",
        "visit_fn_arg",
        "visit_foreign_item_fn",
        "visit_foreign_item_static",
        "visit_foreign_item_type",
        "visit_item_const",
        "visit_item_enum",
        "visit_item_extern_crate",
        "visit_item_fn",
        "visit_item_impl",
        "visit_item_macro",
        "visit_item_mod",
        "visit_item_static",
        "visit_item_struct",
        "visit_item_trait",
        "visit_item_trait_alias",
        "visit_item_type",
        "visit_item_union",
        "visit_label",
        "visit_lifetime_param",
        "visit_local",
        "visit_pat_ident",
        "visit_type_param",
        "visit_use_rename",
        "visit_variant",
    ];
}

impl TryFrom<Syntax> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Syntax) -> Result<Self, Self::Error> {
        let foreign_name_text = Syntax::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Syntax::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_syntax_is_a_name() {
        let syntax = Syntax;

        let foreign_language = ForeignLanguage::try_from(syntax);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
