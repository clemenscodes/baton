use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ManifestText {
    text: String,
}

impl Layer for ManifestText {
    type Layer = DomainLayer;
}

impl ValueObject for ManifestText {}

impl ManifestText {
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl From<&str> for ManifestText {
    fn from(file_text: &str) -> Self {
        let text = file_text.to_string();
        Self { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_manifest_text_keeps_the_text_it_is_made_of() {
        let file_text = "[package]";
        let manifest_text = ManifestText::from(file_text);

        let text = manifest_text.text();

        assert_eq!(text, file_text);
    }
}
