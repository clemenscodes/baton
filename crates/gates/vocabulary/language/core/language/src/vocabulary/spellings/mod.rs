use crate::Code;
use crate::Terms;
use crate::identifier::Identifier;
use crate::vocabulary::Names;
use crate::vocabulary::term::Term;
use crate::vocabulary::term::spelling::Spelling;
use crate::vocabulary::word;
use crate::vocabulary::word::Word;
use stack_domain_driven_design_core::entity::Entity;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::ops::BitOr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Spellings {
    spellings: BTreeSet<Spelling>,
}

impl Layer for Spellings {
    type Layer = DomainLayer;
}

impl ValueObject for Spellings {}

impl Spellings {
    fn decide_piece(&self, piece_words: &[Word]) -> CandidateDecision {
        let Self {
            spellings: piece_spellings,
        } = Self::from((self, piece_words));
        match piece_spellings.is_empty() {
            true => CandidateDecision::Rejection,
            false => CandidateDecision::Acceptance,
        }
    }

    fn decide_words(&self, identifier_words: &[Word]) -> CandidateDecision {
        let word_count = identifier_words.len();
        let decision_count = word_count + 1;
        let rejection = CandidateDecision::Rejection;
        let acceptance = CandidateDecision::Acceptance;
        let mut end_decisions = vec![rejection; decision_count];
        end_decisions[0] = acceptance;
        for piece_start in 0..word_count {
            let start_decision = end_decisions[piece_start];
            for piece_end in piece_start + 1..=word_count {
                let piece_words = &identifier_words[piece_start..piece_end];
                let piece_decision = self.decide_piece(piece_words);
                let piece_end_decision = start_decision & piece_decision;
                let end_decision = end_decisions[piece_end];
                end_decisions[piece_end] = end_decision | piece_end_decision;
            }
        }
        end_decisions[word_count]
    }

    fn select_start_spellings(
        &self,
        identifier_words: &[Word],
        piece_start: usize,
    ) -> BTreeSet<Spelling> {
        let front_words = &identifier_words[..piece_start];
        let front_word_decision = self.decide_words(front_words);
        let mut start_spellings = BTreeSet::new();
        for piece_end in piece_start + 1..=identifier_words.len() {
            let piece_words = &identifier_words[piece_start..piece_end];
            let back_words = &identifier_words[piece_end..];
            let back_word_decision = self.decide_words(back_words);
            let decision = front_word_decision & back_word_decision;
            match decision {
                CandidateDecision::Acceptance => {
                    let Self {
                        spellings: piece_spellings,
                    } = Self::from((self, piece_words));
                    start_spellings.extend(piece_spellings);
                }
                CandidateDecision::Rejection => {}
            }
        }
        start_spellings
    }
}

impl Specification<Identifier> for Spellings {
    fn decide_acceptance(&self, candidate: &Identifier) -> CandidateDecision {
        let identifier_words = candidate.as_ref();
        self.decide_words(identifier_words)
    }
}

impl Specification<Spelling> for Spellings {
    fn decide_acceptance(&self, candidate: &Spelling) -> CandidateDecision {
        let Self { spellings } = self;
        match spellings.contains(candidate) {
            true => CandidateDecision::Acceptance,
            false => CandidateDecision::Rejection,
        }
    }
}

impl FromIterator<Spelling> for Spellings {
    fn from_iter<SpellingsType: IntoIterator<Item = Spelling>>(
        spelling_sequence: SpellingsType,
    ) -> Self {
        let spellings = spelling_sequence.into_iter().collect();
        Self { spellings }
    }
}

impl From<&Terms> for Spellings {
    fn from(terms: &Terms) -> Self {
        terms.into_iter().map(Term::identity).cloned().collect()
    }
}

impl From<&Names> for Spellings {
    fn from(names: &Names) -> Self {
        names.into_iter().map(Spelling::from).collect()
    }
}

impl From<(&Spellings, &[Word])> for Spellings {
    fn from((vocabulary_spellings, piece_words): (&Spellings, &[Word])) -> Self {
        let Spellings { spellings } = vocabulary_spellings;
        let last_position = piece_words.len() - 1;
        let last_word = &piece_words[last_position];
        let leading_words = &piece_words[..last_position];
        let stems = last_word.compose_stems();
        let last_words = [last_word];
        let last_word_forms = stems.into_iter().chain(last_words);
        let mut piece_spellings = BTreeSet::new();
        for last_word_form in last_word_forms {
            let mut spelling_word_list = leading_words.to_vec();
            let spelling_word = last_word_form.clone();
            spelling_word_list.push(spelling_word);
            let spelling_words = spelling_word_list.as_slice();
            if let Some(spelling) = spellings.get(spelling_words) {
                let piece_spelling = spelling.clone();
                piece_spellings.insert(piece_spelling);
            }
        }
        Self {
            spellings: piece_spellings,
        }
    }
}

impl From<(&Spellings, &Identifier)> for Spellings {
    fn from((spellings, identifier): (&Spellings, &Identifier)) -> Self {
        let identifier_words = identifier.as_ref();
        let word_count = identifier_words.len();
        let mut identifier_spellings = BTreeSet::new();
        for piece_start in 0..word_count {
            let start_spellings = spellings.select_start_spellings(identifier_words, piece_start);
            identifier_spellings.extend(start_spellings);
        }
        Self {
            spellings: identifier_spellings,
        }
    }
}

impl From<(&Spellings, &Code)> for Spellings {
    fn from((spellings, code): (&Spellings, &Code)) -> Self {
        let identifiers = code.identifiers();
        let declarations = code.declarations();
        let occurrences = identifiers.into_iter().chain(declarations);
        let mut code_spellings = BTreeSet::new();
        for occurrence in occurrences {
            let identifier = occurrence.identifier();
            let Self {
                spellings: identifier_spellings,
            } = Self::from((spellings, identifier));
            code_spellings.extend(identifier_spellings);
        }
        Self {
            spellings: code_spellings,
        }
    }
}

impl TryFrom<&[&str]> for Spellings {
    type Error = word::Error;

    fn try_from(word_texts: &[&str]) -> Result<Self, Self::Error> {
        let mut spelling_list = Vec::new();
        for word_text in word_texts.iter().copied() {
            let word = Word::try_from(word_text)?;
            let spelling = Spelling::from(word);
            spelling_list.push(spelling);
        }
        let spellings = spelling_list.into_iter().collect();
        Ok(spellings)
    }
}

impl BitOr for &Spellings {
    type Output = Spellings;

    fn bitor(self, spellings: Self) -> Self::Output {
        let Spellings {
            spellings: augend_spellings,
        } = self;
        let Spellings {
            spellings: addend_spellings,
        } = spellings;
        augend_spellings.union(addend_spellings).cloned().collect()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::TermText;
    use crate::identifier::tests::fixture as identifier_fixture;
    use crate::tests::fixture as source_fixture;

    pub(crate) mod fixture {
        use super::*;

        pub(crate) fn word_spellings_factory(word_texts: &[&str]) -> Spellings {
            Spellings::try_from(word_texts).unwrap()
        }

        pub(crate) fn term_spellings_factory(spelling_text: &'static [&'static str]) -> Spellings {
            let definition_text = "A definition.";
            let term_text: TermText = TermText::new(spelling_text, definition_text);
            let term_text_list = [term_text];
            let term_texts = term_text_list.as_slice();
            let terms = Terms::try_from(term_texts).unwrap();
            Spellings::from(&terms)
        }
    }

    #[test]
    fn a_word_of_the_spellings_is_admitted() {
        let word_texts: &[&str] = &["treasure"];
        let identifier_text = "Treasure";
        let spellings = fixture::word_spellings_factory(word_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn an_identifier_of_admitted_words_alone_is_admitted() {
        let word_texts: &[&str] = &["anchor", "chain"];
        let identifier_text = "anchor_chain";
        let spellings = fixture::word_spellings_factory(word_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_word_outside_the_spellings_is_rejected() {
        let word_texts: &[&str] = &["treasure"];
        let identifier_text = "anchor";
        let spellings = fixture::word_spellings_factory(word_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let rejection = CandidateDecision::Rejection;
        assert_eq!(decision, rejection);
    }

    #[test]
    fn a_term_admits_the_identifier_spelled_by_all_of_its_words() {
        let spelling_text: &'static [&'static str] = &["Harbor", "Master"];
        let identifier_text = "HarborMaster";
        let spellings = fixture::term_spellings_factory(spelling_text);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_term_admits_none_of_its_words_alone() {
        let spelling_text: &'static [&'static str] = &["Harbor", "Master"];
        let identifier_text = "harbor";
        let spellings = fixture::term_spellings_factory(spelling_text);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let rejection = CandidateDecision::Rejection;
        assert_eq!(decision, rejection);
    }

    #[test]
    fn a_term_admits_no_identifier_whose_ending_is_another_word() {
        let spelling_text: &'static [&'static str] = &["Treasure", "Chest"];
        let identifier_text = "treasure_anchor";
        let spellings = fixture::term_spellings_factory(spelling_text);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let rejection = CandidateDecision::Rejection;
        assert_eq!(decision, rejection);
    }

    #[test]
    fn the_last_word_of_a_term_may_stand_in_the_plural() {
        let spelling_text: &'static [&'static str] = &["Treasure", "Chest"];
        let identifier_text = "treasure_chests";
        let spellings = fixture::term_spellings_factory(spelling_text);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_word_inside_a_term_may_not_stand_in_the_plural() {
        let spelling_text: &'static [&'static str] = &["Treasure", "Chest"];
        let identifier_text = "treasures_chest";
        let spellings = fixture::term_spellings_factory(spelling_text);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let rejection = CandidateDecision::Rejection;
        assert_eq!(decision, rejection);
    }

    #[test]
    fn an_admitted_word_may_stand_in_the_plural() {
        let word_texts: &[&str] = &["entry"];
        let identifier_text = "entries";
        let spellings = fixture::word_spellings_factory(word_texts);
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn an_identifier_may_be_spelled_by_several_terms_one_after_another() {
        let term_spelling_text: &'static [&'static str] = &["Treasure", "Chest"];
        let word_spelling_text: &'static [&'static str] = &["Anchor"];
        let identifier_text = "anchor_treasure_chest";
        let term_spellings = fixture::term_spellings_factory(term_spelling_text);
        let word_spellings = fixture::term_spellings_factory(word_spelling_text);
        let spellings = &term_spellings | &word_spellings;
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let decision = spellings.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_word_text_that_is_no_word_is_an_error() {
        let word_texts: &[&str] = &["not a word"];

        let spellings = Spellings::try_from(word_texts);

        let text_literal = "not a word";
        let text = String::from(text_literal);
        let error = word::Error::Word { text };
        let expectation = Err(error);
        assert_eq!(spellings, expectation);
    }

    #[test]
    fn the_same_word_in_two_writings_is_one_spelling() {
        let word_texts: &[&str] = &["seat", "Seat"];
        let word_texts_copy: &[&str] = &["SEAT"];
        let spellings = fixture::word_spellings_factory(word_texts);

        let spellings_copy = Spellings::try_from(word_texts_copy);

        let expectation = Ok(spellings);
        assert_eq!(spellings_copy, expectation);
    }

    #[test]
    fn the_spellings_of_a_code_are_those_that_spell_an_identifier_of_it_without_a_gap() {
        let word_texts: &[&str] = &["treasure", "chest", "anchor"];
        let identifier_texts: &[&str] = &["treasure_chest", "anchor_zonk"];
        let spellings = fixture::word_spellings_factory(word_texts);
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let code_spellings = Spellings::from((&spellings, production_code));

        let expectation_texts: &[&str] = &["treasure", "chest"];
        let expectation = fixture::word_spellings_factory(expectation_texts);
        assert_eq!(code_spellings, expectation);
    }

    #[test]
    fn the_names_of_a_vocabulary_spell_a_compound_of_them() {
        let name_texts: &[&str] = &["is_empty"];
        let identifier_text = "is_empty_is_empty";
        let names = Names::try_from(name_texts).unwrap();
        let identifier = identifier_fixture::identifier_factory(identifier_text);

        let name_spellings = Spellings::from(&names);

        let decision = name_spellings.decide_acceptance(&identifier);
        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }
}
