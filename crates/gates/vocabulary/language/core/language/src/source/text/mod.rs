use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SourceText {
    text: String,
}

impl Layer for SourceText {
    type Layer = DomainLayer;
}

impl ValueObject for SourceText {}

impl SourceText {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl From<&str> for SourceText {
    fn from(file_text: &str) -> Self {
        let text = file_text.to_string();
        Self { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_source_text_keeps_the_text_it_is_made_of() {
        let file_text = "struct Treasure;";
        let source_text = SourceText::from(file_text);

        let text = source_text.text();

        assert_eq!(text, file_text);
    }
}
