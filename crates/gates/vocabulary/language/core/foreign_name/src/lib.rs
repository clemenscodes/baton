use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ForeignName {
    text: String,
}

impl Layer for ForeignName {
    type Layer = DomainLayer;
}

impl ValueObject for ForeignName {}

impl From<&str> for ForeignName {
    fn from(name_text: &str) -> Self {
        let text = name_text.to_string();
        Self { text }
    }
}

impl fmt::Display for ForeignName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { text } = self;
        fmt::Display::fmt(text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_foreign_names_of_the_same_text_are_the_same_foreign_name() {
        let foreign_name_text = "clap";
        let foreign_name_text_copy = "clap";
        let foreign_name = ForeignName::from(foreign_name_text);

        let foreign_name_copy = ForeignName::from(foreign_name_text_copy);

        assert_eq!(foreign_name, foreign_name_copy);
    }

    #[test]
    fn a_foreign_name_is_shown_as_its_text() {
        let foreign_name_text = "clap";
        let foreign_name = ForeignName::from(foreign_name_text);

        let text = foreign_name.to_string();

        assert_eq!(text, foreign_name_text);
    }
}
