mod names;
mod spellings;
mod stems;

pub mod name;
pub mod term;
pub mod word;

pub(crate) use names::Names;
pub use spellings::Spellings;

use crate::Code;
use crate::UnknownVocabulary;
use crate::identifier::Identifier;
use crate::occurrence::Occurrence;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::ops::BitOr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Vocabulary {
    spellings: Spellings,
    names: Names,
}

impl Layer for Vocabulary {
    type Layer = DomainLayer;
}

impl ValueObject for Vocabulary {}

impl Vocabulary {
    pub fn spellings(&self) -> &Spellings {
        match self {
            Self { spellings, .. } => spellings,
        }
    }

    pub fn select_unknown_vocabulary(&self, code: &Code) -> UnknownVocabulary {
        let Self { spellings, .. } = self;
        let identifiers = code.identifiers();
        let declarations = code.declarations();
        let unknown_occurrences = Self::select_unknown_occurrences(self, identifiers);
        let unknown_declarations = Self::select_unknown_occurrences(spellings, declarations);
        unknown_occurrences
            .into_iter()
            .chain(unknown_declarations)
            .collect()
    }

    fn select_unknown_occurrences<'code>(
        specification: &impl Specification<Identifier>,
        occurrences: impl IntoIterator<Item = &'code Occurrence>,
    ) -> Vec<Occurrence> {
        let mut unknown_occurrences = Vec::new();
        for occurrence in occurrences {
            let occurrence_identifier = occurrence.identifier();
            let decision = specification.decide_acceptance(occurrence_identifier);
            match decision {
                CandidateDecision::Acceptance => {}
                CandidateDecision::Rejection => {
                    let unknown_occurrence = occurrence.clone();
                    unknown_occurrences.push(unknown_occurrence);
                }
            }
        }
        unknown_occurrences
    }

    pub fn select_unknown_test_vocabulary(&self, code: &Code) -> UnknownVocabulary {
        let Self { spellings, names } = self;
        let name_spellings = Spellings::from(names);
        let test_spellings = spellings | &name_spellings;
        let identifiers = code.identifiers();
        let declarations = code.declarations();
        let occurrences = identifiers.into_iter().chain(declarations);
        let mut unknown_occurrences = Vec::new();
        for occurrence in occurrences {
            let occurrence_identifier = occurrence.identifier();
            let decision = test_spellings.decide_acceptance(occurrence_identifier);
            match decision {
                CandidateDecision::Acceptance => {}
                CandidateDecision::Rejection => {
                    let unknown_occurrence = occurrence.clone();
                    unknown_occurrences.push(unknown_occurrence);
                }
            }
        }
        unknown_occurrences.into_iter().collect()
    }
}

impl Specification<Identifier> for Vocabulary {
    fn decide_acceptance(&self, candidate: &Identifier) -> CandidateDecision {
        let Self { spellings, names } = self;
        let name_decision = names.decide_acceptance(candidate);
        let spelling_decision = spellings.decide_acceptance(candidate);
        name_decision | spelling_decision
    }
}

impl From<Spellings> for Vocabulary {
    fn from(spellings: Spellings) -> Self {
        let names = Names::default();
        Self { spellings, names }
    }
}

impl From<Names> for Vocabulary {
    fn from(names: Names) -> Self {
        let spellings = Spellings::default();
        Self { spellings, names }
    }
}

impl BitOr for &Vocabulary {
    type Output = Vocabulary;

    fn bitor(self, vocabulary: Self) -> Self::Output {
        let Vocabulary {
            spellings: augend_spellings,
            names: augend_names,
        } = self;
        let Vocabulary {
            spellings: addend_spellings,
            names: addend_names,
        } = vocabulary;
        let spellings = augend_spellings | addend_spellings;
        let names = augend_names | addend_names;
        Vocabulary { spellings, names }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::tests::fixture as identifier_fixture;
    use crate::tests::fixture as source_fixture;
    use crate::vocabulary::spellings::tests::fixture as spellings_fixture;

    mod fixture {
        use super::*;

        pub(super) fn name_vocabulary_factory(name_texts: &[&str]) -> Vocabulary {
            let names = Names::try_from(name_texts).unwrap();
            Vocabulary::from(names)
        }

        pub(super) fn word_vocabulary_factory(word_texts: &[&str]) -> Vocabulary {
            let spellings = spellings_fixture::word_spellings_factory(word_texts);
            Vocabulary::from(spellings)
        }
    }

    #[test]
    fn a_word_of_the_vocabulary_admits_its_identifier() {
        let word_texts: &[&str] = &["treasure"];
        let identifier_text = "Treasure";
        let vocabulary = fixture::word_vocabulary_factory(word_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = vocabulary.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_name_of_the_vocabulary_admits_the_identifier_of_exactly_its_words() {
        let name_texts: &[&str] = &["is_empty"];
        let identifier_text = "is_empty";
        let vocabulary = fixture::name_vocabulary_factory(name_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = vocabulary.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_word_of_an_admitted_name_is_no_word_of_its_own() {
        let name_texts: &[&str] = &["is_empty"];
        let identifier_text = "is_port";
        let vocabulary = fixture::name_vocabulary_factory(name_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = vocabulary.decide_acceptance(&identifier);

        let rejection = CandidateDecision::Rejection;
        assert_eq!(decision, rejection);
    }

    #[test]
    fn a_vocabulary_joined_with_another_admits_what_either_admits() {
        let word_texts: &[&str] = &["treasure"];
        let name_texts: &[&str] = &["anchor"];
        let identifier_text = "anchor";
        let word_vocabulary = fixture::word_vocabulary_factory(word_texts);
        let name_vocabulary = fixture::name_vocabulary_factory(name_texts);
        let vocabulary = &word_vocabulary | &name_vocabulary;
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = vocabulary.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn an_identifier_a_source_names_is_unknown_where_neither_a_name_nor_a_word_admits_it() {
        let name_texts: &[&str] = &["is_empty"];
        let identifier_texts: &[&str] = &["is_empty", "anchor"];
        let vocabulary = fixture::name_vocabulary_factory(name_texts);
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:2: `anchor`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn an_identifier_a_source_declares_is_unknown_where_only_a_name_admits_it() {
        let name_texts: &[&str] = &["is_empty"];
        let declaration_texts: &[&str] = &["is_empty"];
        let vocabulary = fixture::name_vocabulary_factory(name_texts);
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:1: `is_empty`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn an_identifier_a_source_declares_is_admitted_where_its_words_spell_it() {
        let word_texts: &[&str] = &["treasure", "chest"];
        let declaration_texts: &[&str] = &["treasure_chest"];
        let vocabulary = fixture::word_vocabulary_factory(word_texts);
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let production_code = source.production_code();

        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);

        let expectation = UnknownVocabulary::default();
        assert_eq!(unknown_vocabulary, expectation);
    }

    #[test]
    fn a_vocabulary_lends_its_spellings() {
        let word_texts: &[&str] = &["treasure"];
        let vocabulary = fixture::word_vocabulary_factory(word_texts);

        let spellings = vocabulary.spellings();

        let expectation = spellings_fixture::word_spellings_factory(word_texts);
        assert_eq!(spellings, &expectation);
    }

    #[test]
    fn an_identifier_of_the_test_code_is_unknown_where_neither_words_nor_names_spell_it() {
        let word_texts: &[&str] = &["treasure"];
        let name_texts: &[&str] = &["is_empty"];
        let identifier_texts: &[&str] = &["is_empty_treasure", "anchor"];
        let word_vocabulary = fixture::word_vocabulary_factory(word_texts);
        let name_vocabulary = fixture::name_vocabulary_factory(name_texts);
        let vocabulary = &word_vocabulary | &name_vocabulary;
        let source = source_fixture::test_identifier_source_factory(identifier_texts);
        let test_code = source.test_code();

        let unknown_vocabulary = vocabulary.select_unknown_test_vocabulary(test_code);

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:2: `anchor`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }
}
