use crate::layer::DomainLayer;
use crate::layer::Layer;
use crate::value_object::ValueObject;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Not;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum CandidateDecision {
    Acceptance,
    Rejection,
}

impl Layer for CandidateDecision {
    type Layer = DomainLayer;
}

impl ValueObject for CandidateDecision {}

impl BitAnd for CandidateDecision {
    type Output = Self;

    fn bitand(self, right_decision: Self) -> Self::Output {
        match self {
            Self::Acceptance => right_decision,
            Self::Rejection => Self::Rejection,
        }
    }
}

impl BitOr for CandidateDecision {
    type Output = Self;

    fn bitor(self, right_decision: Self) -> Self::Output {
        match self {
            Self::Acceptance => Self::Acceptance,
            Self::Rejection => right_decision,
        }
    }
}

impl Not for CandidateDecision {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Acceptance => Self::Rejection,
            Self::Rejection => Self::Acceptance,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_acceptance_and_a_decision_are_that_decision() {
        let acceptance = CandidateDecision::Acceptance;
        let rejection = CandidateDecision::Rejection;

        let candidate_decision = acceptance & rejection;

        let expectation = CandidateDecision::Rejection;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_rejection_and_a_decision_are_a_rejection() {
        let rejection = CandidateDecision::Rejection;
        let acceptance = CandidateDecision::Acceptance;

        let candidate_decision = rejection & acceptance;

        let expectation = CandidateDecision::Rejection;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn an_acceptance_or_a_decision_is_an_acceptance() {
        let acceptance = CandidateDecision::Acceptance;
        let rejection = CandidateDecision::Rejection;

        let candidate_decision = acceptance | rejection;

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_rejection_or_a_decision_is_that_decision() {
        let rejection = CandidateDecision::Rejection;
        let acceptance = CandidateDecision::Acceptance;

        let candidate_decision = rejection | acceptance;

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn the_negation_of_an_acceptance_is_a_rejection() {
        let acceptance = CandidateDecision::Acceptance;

        let candidate_decision = !acceptance;

        let expectation = CandidateDecision::Rejection;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn the_negation_of_a_rejection_is_an_acceptance() {
        let rejection = CandidateDecision::Rejection;

        let candidate_decision = !rejection;

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(candidate_decision, expectation);
    }
}
