use crate::vocabulary::Spellings;
use crate::vocabulary::term::spelling::Spelling;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeMap;
use std::collections::btree_map::Iter;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SourceSpellings {
    spellings: BTreeMap<PathBuf, Spellings>,
}

impl Layer for SourceSpellings {
    type Layer = DomainLayer;
}

impl ValueObject for SourceSpellings {}

impl From<BTreeMap<PathBuf, Spellings>> for SourceSpellings {
    fn from(spellings: BTreeMap<PathBuf, Spellings>) -> Self {
        Self { spellings }
    }
}

impl<'source_spellings> IntoIterator for &'source_spellings SourceSpellings {
    type Item = (&'source_spellings PathBuf, &'source_spellings Spellings);
    type IntoIter = Iter<'source_spellings, PathBuf, Spellings>;

    fn into_iter(self) -> Self::IntoIter {
        let SourceSpellings { spellings } = self;
        spellings.iter()
    }
}

impl Specification<Spelling> for SourceSpellings {
    fn decide_acceptance(&self, candidate: &Spelling) -> CandidateDecision {
        let Self { spellings } = self;
        let mut decision = CandidateDecision::Rejection;
        for source_spellings in spellings.values() {
            let source_decision = source_spellings.decide_acceptance(candidate);
            decision = decision | source_decision;
        }
        decision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

        pub(super) fn source_spellings_factory() -> SourceSpellings {
            let first_word_texts: &[&str] = &["anchor"];
            let last_word_texts: &[&str] = &["treasure"];
            let first_spellings = Spellings::try_from(first_word_texts).unwrap();
            let last_spellings = Spellings::try_from(last_word_texts).unwrap();
            let first_path_text = "crates/harbor/src/lib.rs";
            let last_path_text = "crates/ship/src/lib.rs";
            let first_path = PathBuf::from(first_path_text);
            let last_path = PathBuf::from(last_path_text);
            let first_entry = (first_path, first_spellings);
            let last_entry = (last_path, last_spellings);
            let path_spelling_list = [last_entry, first_entry];
            let path_spellings = BTreeMap::from(path_spelling_list);
            SourceSpellings::from(path_spellings)
        }
    }

    #[test]
    fn a_spelling_one_source_spells_is_accepted() {
        let source_spellings = fixture::source_spellings_factory();
        let spelling_texts: &[&str] = &["Treasure"];
        let spelling = Spelling::try_from(spelling_texts).unwrap();

        let decision = source_spellings.decide_acceptance(&spelling);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_spelling_no_source_spells_is_rejected() {
        let source_spellings = fixture::source_spellings_factory();
        let spelling_texts: &[&str] = &["Chest"];
        let spelling = Spelling::try_from(spelling_texts).unwrap();

        let decision = source_spellings.decide_acceptance(&spelling);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn source_spellings_yield_their_paths_in_order() {
        let source_spellings = fixture::source_spellings_factory();

        let path_texts: Vec<String> = source_spellings
            .into_iter()
            .map(|(path, _)| path.display().to_string())
            .collect();

        let expectation = ["crates/harbor/src/lib.rs", "crates/ship/src/lib.rs"];
        assert_eq!(path_texts, expectation);
    }
}
