use crate::Error;
use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_language::Language;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ForeignLanguage {
    foreign_name: ForeignName,
    language: Language,
}

impl Layer for ForeignLanguage {
    type Layer = DomainLayer;
}

impl ValueObject for ForeignLanguage {}

impl ForeignLanguage {
    pub(super) fn foreign_name(&self) -> &ForeignName {
        &self.foreign_name
    }

    pub(super) fn language(&self) -> &Language {
        &self.language
    }
}

impl TryFrom<(ForeignName, &[&str])> for ForeignLanguage {
    type Error = Error;

    fn try_from((foreign_name, name_texts): (ForeignName, &[&str])) -> Result<Self, Self::Error> {
        let language = Language::try_from(name_texts)?;
        let foreign_language = Self {
            foreign_name,
            language,
        };
        Ok(foreign_language)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_core_language::vocabulary::name;
    use gates_vocabulary_language_core_language::vocabulary::word;

    #[test]
    fn a_foreign_language_of_a_text_that_is_no_name_is_an_error() {
        let name_texts: &[&str] = &["is empty"];
        let foreign_name_text = "anchor";
        let foreign_name = ForeignName::from(foreign_name_text);

        let foreign_language = ForeignLanguage::try_from((foreign_name, name_texts));

        let text_literal = "is empty";
        let text = String::from(text_literal);
        let word_text_literal = "is empty";
        let word_text = String::from(word_text_literal);
        let word_error = word::Error::Word { text: word_text };
        let name_error = name::Error::Word {
            text,
            error: word_error,
        };
        let language_error = gates_vocabulary_language_core_language::Error::Name {
            error: name_error,
        };
        let error = Error::from(language_error);
        let expectation = Err(error);
        assert_eq!(foreign_language, expectation);
    }
}
