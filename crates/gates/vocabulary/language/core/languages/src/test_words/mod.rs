use gates_vocabulary_language_core_language::Error;
use gates_vocabulary_language_core_language::Language;
use gates_vocabulary_language_core_language::vocabulary::{Spellings, Vocabulary};
use stack_domain_driven_design_core::factory::Factory;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct TestWords {
    word_texts: &'static [&'static str],
}

impl Layer for TestWords {
    type Layer = DomainLayer;
}

impl Factory<Language> for TestWords {
    type Blueprint<'blueprint> = ();
    type Error = Error;

    fn try_create_product(&self, _: ()) -> Result<Language, Error> {
        let Self { word_texts } = self;
        let test_word_texts: &[&str] = word_texts;
        let spellings = Spellings::try_from(test_word_texts)?;
        let vocabulary = Vocabulary::from(spellings);
        let language = Language::from(vocabulary);
        Ok(language)
    }
}

impl TestWords {
    const TEST_WORDS: &'static [&'static str] = &[
        "tests",
        "fixture",
        "helper",
        "factory",
        "expectation",
    ];
}

impl From<&'static [&'static str]> for TestWords {
    fn from(word_texts: &'static [&'static str]) -> Self {
        Self { word_texts }
    }
}

impl From<()> for TestWords {
    fn from(_: ()) -> Self {
        let word_texts = Self::TEST_WORDS;
        Self::from(word_texts)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture as source_fixture;
    use gates_vocabulary_language_core_language::vocabulary::word;

    #[test]
    fn every_test_word_is_a_word() {
        let test_words = TestWords::from(());

        let language = test_words.try_create_product(());

        let error = language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }

    #[test]
    fn the_test_words_spell_the_identifiers_a_test_module_declares_of_them() {
        let test_words = TestWords::from(());
        let declaration_texts: &[&str] = &[
            "tests",
            "fixture",
            "helper",
            "source_factory",
            "expectation",
        ];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let production_code = source.production_code();

        let language = test_words.try_create_product(()).unwrap();

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:4: `source_factory`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_test_word_that_is_no_word_is_an_error() {
        let word_texts: &'static [&'static str] = &["test word"];
        let test_words = TestWords::from(word_texts);

        let language = test_words.try_create_product(());

        let text_literal = "test word";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let error = Error::Word { error: word_error };
        let expectation = Err(error);
        assert_eq!(language, expectation);
    }
}
