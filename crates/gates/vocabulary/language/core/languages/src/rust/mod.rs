mod keywords;
mod standard_library;
mod words;

use crate::Error;
use crate::rust::keywords::Keywords;
use crate::rust::standard_library::StandardLibrary;
use crate::rust::words::Words;
use gates_vocabulary_language_core_language::Language;
use stack_domain_driven_design_core::factory::Factory;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct Rust {
    language: Language,
}

impl Layer for Rust {
    type Layer = DomainLayer;
}

impl ValueObject for Rust {}

impl Rust {
    pub(super) fn language(&self) -> &Language {
        match self {
            Self { language } => language,
        }
    }
}

impl TryFrom<(Keywords, StandardLibrary, Words)> for Rust {
    type Error = Error;

    fn try_from(
        (keywords, standard_library, words): (Keywords, StandardLibrary, Words),
    ) -> Result<Self, Self::Error> {
        let keyword_language = keywords.try_create_product(())?;
        let standard_library_language = standard_library.try_create_product(())?;
        let word_language = words.try_create_product(())?;
        let name_language = &keyword_language | &standard_library_language;
        let language = &name_language | &word_language;
        let rust = Self { language };
        Ok(rust)
    }
}

impl TryFrom<()> for Rust {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let keywords = Keywords::from(());
        let standard_library = StandardLibrary::from(());
        let words = Words::from(());
        Self::try_from((keywords, standard_library, words))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture as source_fixture;
    use gates_vocabulary_language_core_language::UnknownVocabulary;
    use gates_vocabulary_language_core_language::vocabulary::name;
    use gates_vocabulary_language_core_language::vocabulary::word;

    mod fixture {
        use super::*;

        pub(super) fn rust_language_factory() -> Language {
            let rust = Rust::try_from(()).unwrap();
            rust.language().clone()
        }
    }

    #[test]
    fn every_keyword_every_name_and_every_word_of_rust_composes_its_language() {
        let rust = Rust::try_from(());

        let error = rust.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }

    #[test]
    fn a_keyword_of_rust_that_is_no_name_is_an_error() {
        let keyword_texts: &'static [&'static str] = &["_"];
        let name_texts: &'static [&'static str] = &[];
        let word_texts: &'static [&'static str] = &[];
        let keywords = Keywords::from(keyword_texts);
        let standard_library = StandardLibrary::from(name_texts);
        let words = Words::from(word_texts);

        let rust = Rust::try_from((keywords, standard_library, words));

        let text_literal = "_";
        let text = String::from(text_literal);
        let name_error = name::Error::Name { text };
        let language_error = gates_vocabulary_language_core_language::Error::Name {
            error: name_error,
        };
        let error = Error::from(language_error);
        let expectation = Err(error);
        assert_eq!(rust, expectation);
    }

    #[test]
    fn a_name_of_the_standard_library_that_is_no_name_is_an_error() {
        let keyword_texts: &'static [&'static str] = &[];
        let name_texts: &'static [&'static str] = &["_"];
        let word_texts: &'static [&'static str] = &[];
        let keywords = Keywords::from(keyword_texts);
        let standard_library = StandardLibrary::from(name_texts);
        let words = Words::from(word_texts);

        let rust = Rust::try_from((keywords, standard_library, words));

        let text_literal = "_";
        let text = String::from(text_literal);
        let name_error = name::Error::Name { text };
        let language_error = gates_vocabulary_language_core_language::Error::Name {
            error: name_error,
        };
        let error = Error::from(language_error);
        let expectation = Err(error);
        assert_eq!(rust, expectation);
    }

    #[test]
    fn a_word_of_rust_that_is_no_word_is_an_error() {
        let keyword_texts: &'static [&'static str] = &[];
        let name_texts: &'static [&'static str] = &[];
        let word_texts: &'static [&'static str] = &["not a word"];
        let keywords = Keywords::from(keyword_texts);
        let standard_library = StandardLibrary::from(name_texts);
        let words = Words::from(word_texts);

        let rust = Rust::try_from((keywords, standard_library, words));

        let text_literal = "not a word";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let language_error = gates_vocabulary_language_core_language::Error::Word {
            error: word_error,
        };
        let error = Error::from(language_error);
        let expectation = Err(error);
        assert_eq!(rust, expectation);
    }

    #[test]
    fn a_keyword_of_rust_is_known_where_the_code_names_it() {
        let identifier_texts: &[&str] = &["pub", "struct", "Treasure"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:3: `Treasure`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_keyword_of_rust_is_no_word_of_an_identifier() {
        let identifier_texts: &[&str] = &["mut_self", "text_for_word", "where_letter"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:1: `mut_self`",
            "1:2: `text_for_word`",
            "1:3: `where_letter`",
        ];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn the_prefix_try_and_the_word_type_are_words_of_every_identifier() {
        let declaration_texts: &[&str] = &["try_type"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let expectation = UnknownVocabulary::default();
        assert_eq!(unknown_vocabulary, expectation);
    }

    #[test]
    fn a_method_of_std_is_known_where_the_code_names_it() {
        let identifier_texts: &[&str] = &["is_empty", "as_bytes"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let expectation = UnknownVocabulary::default();
        assert_eq!(unknown_vocabulary, expectation);
    }

    #[test]
    fn a_name_of_std_is_unknown_where_the_code_declares_it() {
        let declaration_texts: &[&str] = &["get", "is_empty", "other"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:1: `get`", "1:2: `is_empty`", "1:3: `other`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_type_of_std_is_known_however_its_words_fall_apart() {
        let identifier_texts: &[&str] = &["BTreeMap", "String", "Vec", "u8", "new"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let expectation = UnknownVocabulary::default();
        assert_eq!(unknown_vocabulary, expectation);
    }

    #[test]
    fn a_word_of_a_method_of_std_is_unknown_in_a_name_std_does_not_have() {
        let identifier_texts: &[&str] = &["is", "has", "is_port", "has_files"];
        let language = fixture::rust_language_factory();
        let vocabulary = language.vocabulary();
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:1: `is`",
            "1:2: `has`",
            "1:3: `is_port`",
            "1:4: `has_files`",
        ];
        assert_eq!(unknown_occurrence_texts, expectation);
    }
}
