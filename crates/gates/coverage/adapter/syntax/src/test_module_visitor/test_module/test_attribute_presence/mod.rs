use ::syn::{Attribute, Meta, MetaList};
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub(super) enum TestAttributePresence {
    Presence { opening_line: usize },
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
            let LineColumn {
                line: opening_line, ..
            } = hash_mark_span.start();
            Self::Presence { opening_line }
        } else {
            Self::Absence
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::syn::ItemMod;

    mod fixture {
        use super::*;

        pub(super) fn attribute_factory(text: &str) -> Attribute {
            let ItemMod {
                attrs: attributes, ..
            } = ::syn::parse_str(text).unwrap();
            attributes.into_iter().next().unwrap()
        }
    }

    #[test]
    fn the_test_attribute_is_a_presence_on_the_line_of_its_hash_mark() {
        let text = r#"
#[cfg(test)]
mod tests {}
"#;
        let attribute = fixture::attribute_factory(text);

        let test_attribute_presence = TestAttributePresence::from(&attribute);

        let opening_line: usize = 2;
        let expectation = TestAttributePresence::Presence { opening_line };
        assert_eq!(test_attribute_presence, expectation);
    }

    #[test]
    fn an_attribute_that_is_no_list_is_an_absence() {
        let text = r#"#[doc = "treasure"]
mod tests {}
"#;
        let attribute = fixture::attribute_factory(text);

        let test_attribute_presence = TestAttributePresence::from(&attribute);

        let expectation = TestAttributePresence::Absence;
        assert_eq!(test_attribute_presence, expectation);
    }

    #[test]
    fn a_list_of_another_path_is_an_absence() {
        let text = r#"#[doc(hidden)]
mod tests {}
"#;
        let attribute = fixture::attribute_factory(text);

        let test_attribute_presence = TestAttributePresence::from(&attribute);

        let expectation = TestAttributePresence::Absence;
        assert_eq!(test_attribute_presence, expectation);
    }

    #[test]
    fn a_configuration_of_other_tokens_is_an_absence() {
        let text = r#"#[cfg(feature = "chest")]
mod tests {}
"#;
        let attribute = fixture::attribute_factory(text);

        let test_attribute_presence = TestAttributePresence::from(&attribute);

        let expectation = TestAttributePresence::Absence;
        assert_eq!(test_attribute_presence, expectation);
    }
}
