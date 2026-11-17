use crate::Error;
use crate::UnusedTerms;
use crate::source::SourceSpellings;
use crate::term_text::TermText;
use crate::vocabulary::term::Term;
use stack_domain_driven_design_core::entity::Entity;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::ops::BitOr;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Terms {
    terms: Vec<Term>,
}

impl Layer for Terms {
    type Layer = DomainLayer;
}

impl ValueObject for Terms {}

impl Terms {
    pub fn term_count(&self) -> usize {
        match self {
            Self { terms } => terms.len(),
        }
    }

    pub fn select_unused_terms(&self, source_spellings: &SourceSpellings) -> UnusedTerms {
        let Self { terms } = self;
        let mut unused_term_spellings = Vec::new();
        for term in terms {
            let spelling = term.identity();
            let decision = source_spellings.decide_acceptance(spelling);
            match decision {
                CandidateDecision::Acceptance => {}
                CandidateDecision::Rejection => {
                    let unused_term_spelling = spelling.clone();
                    unused_term_spellings.push(unused_term_spelling);
                }
            }
        }
        unused_term_spellings.into_iter().collect()
    }
}

impl FromIterator<Term> for Terms {
    fn from_iter<TermsType: IntoIterator<Item = Term>>(term_sequence: TermsType) -> Self {
        let mut terms: Vec<Term> = term_sequence.into_iter().collect();
        terms.sort_by_cached_key(ToString::to_string);
        Self { terms }
    }
}

impl TryFrom<&[TermText]> for Terms {
    type Error = Error;

    fn try_from(term_texts: &[TermText]) -> Result<Self, Self::Error> {
        let mut spellings = BTreeSet::new();
        let mut term_list = Vec::new();
        for term_text in term_texts.iter().copied() {
            let term = Term::try_from(term_text)?;
            let spelling = term.identity().clone();
            match spellings.insert(spelling) {
                true => term_list.push(term),
                false => {
                    let error = Error::Redefinition { term_text };
                    return Err(error);
                }
            }
        }
        let terms = term_list.into_iter().collect();
        Ok(terms)
    }
}

impl BitOr for &Terms {
    type Output = Terms;

    fn bitor(self, terms: Self) -> Self::Output {
        let Terms {
            terms: augend_terms,
        } = self;
        let Terms {
            terms: addend_terms,
        } = terms;
        augend_terms.iter().chain(addend_terms).cloned().collect()
    }
}

impl<'terms> IntoIterator for &'terms Terms {
    type Item = &'terms Term;
    type IntoIter = Iter<'terms, Term>;

    fn into_iter(self) -> Self::IntoIter {
        let Terms { terms } = self;
        terms.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocabulary::Spellings;
    use crate::vocabulary::term;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    #[test]
    fn the_terms_of_term_texts_are_ordered_by_their_spelling() {
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

        let terms = Terms::try_from(term_texts).unwrap();

        let spelling_texts: Vec<String> = terms.into_iter().map(ToString::to_string).collect();
        let expectation = ["Chest", "Treasure"];
        assert_eq!(spelling_texts, expectation);
    }

    #[test]
    fn a_spelling_defined_a_second_time_is_an_error() {
        const DEFINITION_TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        const REDEFINITION_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team delivers to the chest.";
            TermText::new(spelling, definition)
        };
        let definition_term_text = DEFINITION_TERM_TEXT;
        let redefinition_text = REDEFINITION_TEXT;
        let term_texts: &[TermText] = &[definition_term_text, redefinition_text];

        let terms = Terms::try_from(term_texts);

        let error = Error::Redefinition {
            term_text: redefinition_text,
        };
        let expectation = Err(error);
        assert_eq!(terms, expectation);
    }

    #[test]
    fn a_spelling_defined_a_second_time_in_another_case_is_an_error() {
        const DEFINITION_TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        const REDEFINITION_TEXT: TermText = {
            let spelling: &[&str] = &["treasure"];
            let definition = "The object a team delivers to the chest.";
            TermText::new(spelling, definition)
        };
        let definition_term_text = DEFINITION_TERM_TEXT;
        let redefinition_text = REDEFINITION_TEXT;
        let term_texts: &[TermText] = &[definition_term_text, redefinition_text];

        let terms = Terms::try_from(term_texts);

        let error = Error::Redefinition {
            term_text: redefinition_text,
        };
        let expectation = Err(error);
        assert_eq!(terms, expectation);
    }

    #[test]
    fn a_term_text_that_is_no_term_is_an_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "A definition.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let term_texts: &[TermText] = &[term_text];

        let terms = Terms::try_from(term_texts);

        let spelling_error = term::spelling::Error::Term;
        let term_error = term::Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let error = Error::Term { error: term_error };
        let expectation = Err(error);
        assert_eq!(terms, expectation);
    }

    #[test]
    fn two_joined_terms_hold_every_term_of_both() {
        const AUGEND_TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        }];
        let augend_term_texts = AUGEND_TERM_TEXTS;
        const ADDEND_TERM_TEXTS: &[TermText] = &[
            {
                let spelling: &[&str] = &["Chest"];
                let definition = "A box.";
                TermText::new(spelling, definition)
            },
            {
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing worth the search.";
                TermText::new(spelling, definition)
            },
        ];
        let addend_term_texts = ADDEND_TERM_TEXTS;
        let augend_terms = Terms::try_from(augend_term_texts).unwrap();
        let addend_terms = Terms::try_from(addend_term_texts).unwrap();

        let terms = &augend_terms | &addend_terms;

        let definition_text = |term: &Term| term.definition().to_string();
        let definition_texts: Vec<String> = terms.into_iter().map(definition_text).collect();
        let expectation = ["A box.", "A thing.", "A thing worth the search."];
        assert_eq!(definition_texts, expectation);
    }

    #[test]
    fn a_term_no_source_spells_is_unused() {
        const TERM_TEXTS: &[TermText] = &[
            {
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing.";
                TermText::new(spelling, definition)
            },
            {
                let spelling: &[&str] = &["Chest"];
                let definition = "A box.";
                TermText::new(spelling, definition)
            },
        ];
        let term_texts = TERM_TEXTS;
        let terms = Terms::try_from(term_texts).unwrap();
        let word_texts: &[&str] = &["treasure"];
        let spellings = Spellings::try_from(word_texts).unwrap();
        let path_text = "crates/ship/src/lib.rs";
        let path = PathBuf::from(path_text);
        let path_spelling_entry = (path, spellings);
        let path_spelling_list = [path_spelling_entry];
        let path_spellings = BTreeMap::from(path_spelling_list);
        let source_spellings = SourceSpellings::from(path_spellings);

        let unused_terms = terms.select_unused_terms(&source_spellings);

        let unused_term_texts: Vec<String> = unused_terms
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["Chest"];
        assert_eq!(unused_term_texts, expectation);
    }

    #[test]
    fn terms_count_the_terms_they_hold() {
        const TERM_TEXTS: &[TermText] = &[
            {
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing.";
                TermText::new(spelling, definition)
            },
            {
                let spelling: &[&str] = &["Chest"];
                let definition = "A box.";
                TermText::new(spelling, definition)
            },
        ];
        let term_texts = TERM_TEXTS;
        let terms = Terms::try_from(term_texts).unwrap();

        let term_count = terms.term_count();

        let expectation: usize = 2;
        assert_eq!(term_count, expectation);
    }
}
