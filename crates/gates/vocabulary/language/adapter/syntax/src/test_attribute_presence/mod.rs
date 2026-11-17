use ::syn::{Attribute, Meta, MetaList};
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub(super) enum TestAttributePresence {
    Presence { start_position: LineColumn },
    Absence,
}

impl Layer for TestAttributePresence {
    type Layer = InfrastructureLayer;
}

impl TestAttributePresence {
    const CONFIGURATION_NAME: &'static str = "cfg";
    const TEST_NAME: &'static str = "test";
}

impl From<&Attribute> for TestAttributePresence {
    fn from(attribute: &Attribute) -> Self {
        let configuration_name = Self::CONFIGURATION_NAME;
        let test_name = Self::TEST_NAME;
        if let Attribute {
            pound_token: hash_mark,
            meta: Meta::List(MetaList {
                path: attribute_path,
                tokens: attribute_tokens,
                ..
            }),
            ..
        } = attribute
            && attribute_path.is_ident(configuration_name)
            && attribute_tokens.to_string() == test_name
        {
            let [hash_mark_span] = hash_mark.spans;
            let start_position = hash_mark_span.start();
            Self::Presence { start_position }
        } else {
            Self::Absence
        }
    }
}
