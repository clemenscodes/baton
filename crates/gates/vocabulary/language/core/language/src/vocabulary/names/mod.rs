use crate::identifier::Identifier;
use crate::vocabulary::name;
use crate::vocabulary::name::Name;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::ops::BitOr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct Names {
    names: BTreeSet<Name>,
}

impl Layer for Names {
    type Layer = DomainLayer;
}

impl ValueObject for Names {}

impl Specification<Identifier> for Names {
    fn decide_acceptance(&self, candidate: &Identifier) -> CandidateDecision {
        let Self { names } = self;
        let candidate_words = candidate.as_ref();
        match names.contains(candidate_words) {
            true => CandidateDecision::Acceptance,
            false => CandidateDecision::Rejection,
        }
    }
}

impl FromIterator<Name> for Names {
    fn from_iter<NamesType: IntoIterator<Item = Name>>(name_sequence: NamesType) -> Self {
        let names = name_sequence.into_iter().collect();
        Self { names }
    }
}

impl TryFrom<&[&str]> for Names {
    type Error = name::Error;

    fn try_from(name_texts: &[&str]) -> Result<Self, Self::Error> {
        let mut name_list = Vec::new();
        for name_text in name_texts.iter().copied() {
            let name = Name::try_from(name_text)?;
            name_list.push(name);
        }
        let names = name_list.into_iter().collect();
        Ok(names)
    }
}

impl BitOr for &Names {
    type Output = Names;

    fn bitor(self, names: Self) -> Self::Output {
        let Names {
            names: augend_names,
        } = self;
        let Names {
            names: addend_names,
        } = names;
        augend_names.union(addend_names).cloned().collect()
    }
}

impl<'names> IntoIterator for &'names Names {
    type Item = &'names Name;
    type IntoIter = Iter<'names, Name>;

    fn into_iter(self) -> Self::IntoIter {
        let Names { names } = self;
        names.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::tests::fixture;
    use crate::vocabulary::word;

    #[test]
    fn a_name_is_admitted_in_exactly_its_words() {
        let name_texts: &[&str] = &["is_empty"];
        let identifier_text = "IsEmpty";
        let names = Names::try_from(name_texts).unwrap();
        let identifier = fixture::identifier_factory(identifier_text);

        let decision = names.decide_acceptance(&identifier);

        let acceptance = CandidateDecision::Acceptance;
        assert_eq!(decision, acceptance);
    }

    #[test]
    fn a_name_text_that_is_no_name_is_an_error() {
        let name_texts: &[&str] = &["is empty"];

        let names = Names::try_from(name_texts);

        let text_literal = "is empty";
        let text = String::from(text_literal);
        let word_text_literal = "is empty";
        let word_text = String::from(word_text_literal);
        let word_error = word::Error::Word { text: word_text };
        let error = name::Error::Word {
            text,
            error: word_error,
        };
        let expectation = Err(error);
        assert_eq!(names, expectation);
    }
}
