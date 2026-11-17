use crate::vocabulary::term::spelling::Spelling;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::CandidateDecision;
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UnusedTerms {
    spellings: BTreeSet<Spelling>,
}

impl Layer for UnusedTerms {
    type Layer = DomainLayer;
}

impl ValueObject for UnusedTerms {}

impl UnusedTerms {
    pub fn decide_acceptance(&self) -> CandidateDecision {
        let Self { spellings } = self;
        match spellings.is_empty() {
            true => CandidateDecision::Acceptance,
            false => CandidateDecision::Rejection,
        }
    }
}

impl FromIterator<Spelling> for UnusedTerms {
    fn from_iter<SpellingsType: IntoIterator<Item = Spelling>>(
        spelling_sequence: SpellingsType,
    ) -> Self {
        let spellings = spelling_sequence.into_iter().collect();
        Self { spellings }
    }
}

impl<'unused_terms> IntoIterator for &'unused_terms UnusedTerms {
    type Item = &'unused_terms Spelling;
    type IntoIter = Iter<'unused_terms, Spelling>;

    fn into_iter(self) -> Self::IntoIter {
        let UnusedTerms { spellings } = self;
        spellings.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocabulary::word::Word;

    #[test]
    fn unused_terms_without_a_spelling_admit_their_context() {
        let unused_terms = UnusedTerms::default();

        let decision = unused_terms.decide_acceptance();

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn unused_terms_of_a_spelling_reject_their_context() {
        let word_text = "Source";
        let word = Word::try_from(word_text).unwrap();
        let spelling = Spelling::from(word);
        let spelling_list = [spelling];
        let unused_terms: UnusedTerms = spelling_list.into_iter().collect();

        let decision = unused_terms.decide_acceptance();

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn unused_terms_lend_their_spellings_in_order() {
        let source_text = "Source";
        let defect_text = "Defect";
        let source_word = Word::try_from(source_text).unwrap();
        let defect_word = Word::try_from(defect_text).unwrap();
        let source_spelling = Spelling::from(source_word);
        let defect_spelling = Spelling::from(defect_word);
        let spelling_list = [source_spelling, defect_spelling];
        let unused_terms: UnusedTerms = spelling_list.into_iter().collect();

        let spelling_texts: Vec<String> = unused_terms
            .into_iter()
            .map(ToString::to_string)
            .collect();

        let expectation = ["Defect", "Source"];
        assert_eq!(spelling_texts, expectation);
    }
}
