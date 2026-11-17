mod error;
mod term_text;
mod terms;
mod unknown_vocabulary;
mod unused_terms;

pub mod code;
pub mod defects;
pub mod identifier;
pub mod occurrence;
pub mod parser;
pub mod reader;
pub mod source;
pub mod vocabulary;

pub use code::Code;
pub use error::Error;
pub use term_text::TermText;
pub use terms::Terms;
pub use unknown_vocabulary::UnknownVocabulary;
pub use unused_terms::UnusedTerms;

use crate::vocabulary::Vocabulary;
use crate::vocabulary::{Names, Spellings};
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::ops::BitOr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Language {
    vocabulary: Vocabulary,
    terms: Terms,
}

impl Layer for Language {
    type Layer = DomainLayer;
}

impl ValueObject for Language {}

impl Language {
    pub fn vocabulary(&self) -> &Vocabulary {
        match self {
            Self { vocabulary, .. } => vocabulary,
        }
    }

    pub fn terms(&self) -> &Terms {
        match self {
            Self { terms, .. } => terms,
        }
    }
}

impl From<Vocabulary> for Language {
    fn from(vocabulary: Vocabulary) -> Self {
        let terms = Terms::default();
        Self { vocabulary, terms }
    }
}

impl TryFrom<&[TermText]> for Language {
    type Error = Error;

    fn try_from(term_texts: &[TermText]) -> Result<Self, Self::Error> {
        let terms = Terms::try_from(term_texts)?;
        let spellings = Spellings::from(&terms);
        let vocabulary = Vocabulary::from(spellings);
        let language = Self { vocabulary, terms };
        Ok(language)
    }
}

impl TryFrom<&[&str]> for Language {
    type Error = Error;

    fn try_from(name_texts: &[&str]) -> Result<Self, Self::Error> {
        let names = Names::try_from(name_texts)?;
        let vocabulary = Vocabulary::from(names);
        let language = Self::from(vocabulary);
        Ok(language)
    }
}

impl BitOr for &Language {
    type Output = Language;

    fn bitor(self, language: Self) -> Self::Output {
        let Language {
            vocabulary: augend_vocabulary,
            terms: augend_terms,
        } = self;
        let Language {
            vocabulary: addend_vocabulary,
            terms: addend_terms,
        } = language;
        let vocabulary = augend_vocabulary | addend_vocabulary;
        let terms = augend_terms | addend_terms;
        Language { vocabulary, terms }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::defects::Defects;
    use crate::defects::defect::Defect;
    use crate::occurrence::Occurrence;
    use crate::source::Source;
    use crate::vocabulary::name;
    use crate::vocabulary::term;
    use crate::vocabulary::word;

    pub(crate) mod fixture {
        use super::*;
        use crate::code::tokens::CodeTokens;
        use crate::code::tokens::code_token::CodeToken;

        pub(crate) const TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Treasure", "Chest"];
            let definition = "A chest that holds a treasure.";
            TermText::new(spelling, definition)
        }];

        fn occurrences_factory<OccurrencesType: FromIterator<Occurrence>>(
            texts: &[&str],
        ) -> OccurrencesType {
            let line: usize = 1;
            let mut occurrence_list = Vec::new();
            for (position, text) in texts.iter().copied().enumerate() {
                let column = position + 1;
                let occurrence = Occurrence::try_from((text, line, column)).unwrap();
                occurrence_list.push(occurrence);
            }
            occurrence_list.into_iter().collect()
        }

        pub(crate) fn identifier_source_factory(identifier_texts: &[&str]) -> Source {
            let declaration_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn declaration_source_factory(declaration_texts: &[&str]) -> Source {
            let identifier_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn test_identifier_source_factory(identifier_texts: &[&str]) -> Source {
            let declaration_texts: &[&str] = &[];
            let production_code = Code::default();
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let test_code = Code::from((identifiers, declarations));
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn token_source_factory() -> Source {
            let production_code = Code::default();
            let line: usize = 1;
            let column: usize = 1;
            let text = String::from("Treasure");
            let code_token = CodeToken::Declaration { line, column, text };
            let code_token_list = [code_token];
            let production_tokens = code_token_list.into_iter().collect();
            let test_code = Code::default();
            let defects = Defects::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn comment_source_factory(identifier_texts: &[&str]) -> Source {
            let declaration_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let line: usize = 2;
            let column: usize = 1;
            let comment = Defect::Comment { line, column };
            let defect_list = [comment];
            let defects = defect_list.into_iter().collect();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }
    }

    #[test]
    fn a_language_of_term_texts_admits_the_identifiers_its_terms_spell() {
        let term_texts = fixture::TERM_TEXTS;
        let identifier_texts: &[&str] = &["treasure_chest", "TreasureChests", "anchor"];
        let language = Language::try_from(term_texts).unwrap();
        let vocabulary = language.vocabulary();
        let source = fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:3: `anchor`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_language_of_term_texts_yields_the_terms_of_every_text() {
        const TERM_TEXTS: &[TermText] = &[
            {
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing worth the search.";
                TermText::new(spelling, definition)
            },
            {
                let spelling: &[&str] = &["Chest"];
                let definition = "A box that holds a treasure.";
                TermText::new(spelling, definition)
            },
        ];
        let term_texts = TERM_TEXTS;

        let language = Language::try_from(term_texts).unwrap();

        let terms = language.terms();
        let spelling_texts: Vec<String> = terms.into_iter().map(ToString::to_string).collect();
        let expectation = ["Chest", "Treasure"];
        assert_eq!(spelling_texts, expectation);
    }

    #[test]
    fn a_language_of_a_term_text_that_is_no_word_is_an_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["not a word"];
            let definition = "A definition.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        const WORD_TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing worth the search.";
            TermText::new(spelling, definition)
        };
        let word_term_text = WORD_TERM_TEXT;
        let term_texts: &[TermText] = &[word_term_text, term_text];

        let language = Language::try_from(term_texts);

        let text_literal = "not a word";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let spelling_error = term::spelling::Error::Word { error: word_error };
        let term_error = term::Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let error = Error::Term { error: term_error };
        let expectation = Err(error);
        assert_eq!(language, expectation);
    }

    #[test]
    fn a_language_of_names_admits_exactly_those_names() {
        let name_texts: &[&str] = &["treasure_chest", "anchor"];
        let identifier_texts: &[&str] = &["treasure_chest", "TreasureChests", "anchor"];
        let language = Language::try_from(name_texts).unwrap();
        let vocabulary = language.vocabulary();
        let source = fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:2: `TreasureChests`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_language_of_names_holds_no_term() {
        let name_texts: &[&str] = &["anchor"];

        let language = Language::try_from(name_texts).unwrap();

        let terms = language.terms();
        let term_expectation = Terms::default();
        let expectation = &term_expectation;
        assert_eq!(terms, expectation);
    }

    #[test]
    fn a_language_of_a_text_that_is_no_name_is_an_error() {
        let name_texts: &[&str] = &["_"];

        let language = Language::try_from(name_texts);

        let text_literal = "_";
        let text = String::from(text_literal);
        let name_error = name::Error::Name { text };
        let error = Error::Name { error: name_error };
        let expectation = Err(error);
        assert_eq!(language, expectation);
    }

    #[test]
    fn a_language_without_a_term_makes_every_identifier_unknown() {
        let identifier_texts: &[&str] = &["treasure_chest", "TreasureChests", "anchor"];
        let language = Language::default();
        let vocabulary = language.vocabulary();
        let source = fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:1: `treasure_chest`",
            "1:2: `TreasureChests`",
            "1:3: `anchor`",
        ];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn two_joined_languages_yield_the_terms_of_both() {
        let term_texts = fixture::TERM_TEXTS;
        let name_texts: &[&str] = &["anchor"];
        let term_language = Language::try_from(term_texts).unwrap();
        let name_language = Language::try_from(name_texts).unwrap();

        let language = &term_language | &name_language;

        let terms = language.terms();
        let spelling_texts: Vec<String> = terms.into_iter().map(ToString::to_string).collect();
        let expectation = ["Treasure Chest"];
        assert_eq!(spelling_texts, expectation);
    }

    #[test]
    fn two_joined_languages_admit_what_either_admits() {
        let term_texts = fixture::TERM_TEXTS;
        let name_texts: &[&str] = &["anchor"];
        let identifier_texts: &[&str] = &["treasure_chest", "anchor"];
        let term_language = Language::try_from(term_texts).unwrap();
        let name_language = Language::try_from(name_texts).unwrap();
        let language = &term_language | &name_language;
        let vocabulary = language.vocabulary();
        let source = fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let expectation = UnknownVocabulary::default();
        assert_eq!(unknown_vocabulary, expectation);
    }
}
