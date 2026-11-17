use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct TermText {
    spelling_text: &'static [&'static str],
    definition_text: &'static str,
}

impl Layer for TermText {
    type Layer = DomainLayer;
}

impl ValueObject for TermText {}

impl TermText {
    const SEPARATOR: &'static str = " ";

    pub const fn new(
        spelling_text: &'static [&'static str],
        definition_text: &'static str,
    ) -> Self {
        Self {
            spelling_text,
            definition_text,
        }
    }

    pub(crate) fn spelling_text(&self) -> &'static [&'static str] {
        self.spelling_text
    }

    pub(crate) fn definition_text(&self) -> &'static str {
        self.definition_text
    }
}

impl fmt::Display for TermText {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            spelling_text,
            definition_text,
        } = self;
        let separator = Self::SEPARATOR;
        let spelling = spelling_text.join(separator);
        write!(formatter, "{spelling}: {definition_text}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_term_text_is_shown_as_its_words_and_its_definition() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure", "Chest"];
            let definition = "A chest that holds a treasure.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;

        let text = term_text.to_string();

        let expectation = "Treasure Chest: A chest that holds a treasure.";
        assert_eq!(text, expectation);
    }
}
