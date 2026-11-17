mod spellings;
mod text;
mod tokens;

pub use spellings::SourceSpellings;
pub use text::SourceText;
pub use tokens::SourceTokens;

use crate::Code;
use crate::code::tokens::CodeTokens;
use crate::defects::Defects;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Source {
    production_code: Code,
    production_tokens: CodeTokens,
    test_code: Code,
    defects: Defects,
}

impl Layer for Source {
    type Layer = DomainLayer;
}

impl ValueObject for Source {}

impl Source {
    pub fn production_code(&self) -> &Code {
        &self.production_code
    }

    pub fn production_tokens(&self) -> &CodeTokens {
        &self.production_tokens
    }

    pub fn test_code(&self) -> &Code {
        match self {
            Self { test_code, .. } => test_code,
        }
    }

    pub fn defects(&self) -> &Defects {
        match self {
            Self { defects, .. } => defects,
        }
    }
}

impl From<(Code, CodeTokens, Code, Defects)> for Source {
    fn from(
        (production_code, production_tokens, test_code, defects): (Code, CodeTokens, Code, Defects),
    ) -> Self {
        Self {
            production_code,
            production_tokens,
            test_code,
            defects,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::fixture;

    #[test]
    fn a_source_lends_the_code_in_front_of_its_test_module() {
        let identifier_texts: &[&str] = &["source"];
        let source = fixture::identifier_source_factory(identifier_texts);

        let production_code = source.production_code();

        let identifier_count = production_code.identifiers().into_iter().count();
        let expectation: usize = 1;
        assert_eq!(identifier_count, expectation);
    }

    #[test]
    fn a_source_lends_the_tokens_in_front_of_its_test_module() {
        let source = fixture::token_source_factory();

        let production_tokens = source.production_tokens();

        let token_count = production_tokens.into_iter().count();
        let expectation: usize = 1;
        assert_eq!(token_count, expectation);
    }

    #[test]
    fn a_source_lends_the_code_of_its_test_module() {
        let identifier_texts: &[&str] = &["source"];
        let source = fixture::test_identifier_source_factory(identifier_texts);

        let test_code = source.test_code();

        let identifier_count = test_code.identifiers().into_iter().count();
        let expectation: usize = 1;
        assert_eq!(identifier_count, expectation);
    }

    #[test]
    fn a_source_lends_the_defects_it_holds() {
        let identifier_texts: &[&str] = &[];
        let source = fixture::comment_source_factory(identifier_texts);

        let defects = source.defects();

        let defect_count = defects.into_iter().count();
        let expectation: usize = 1;
        assert_eq!(defect_count, expectation);
    }
}
