use crate::layer::DomainLayer;
use crate::layer::Layer;
use crate::specification::CandidateDecision;
use crate::specification::Conjunction;
use crate::specification::Negation;
use crate::specification::Specification;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Not;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Disjunction<LeftSpecification, RightSpecification> {
    left_specification: LeftSpecification,
    right_specification: RightSpecification,
}

impl<LeftSpecification, RightSpecification> From<(LeftSpecification, RightSpecification)>
    for Disjunction<LeftSpecification, RightSpecification>
{
    fn from(
        (left_specification, right_specification): (LeftSpecification, RightSpecification),
    ) -> Self {
        Self {
            left_specification,
            right_specification,
        }
    }
}

impl<LeftSpecification, RightSpecification> Layer
    for Disjunction<LeftSpecification, RightSpecification>
where
    LeftSpecification: Layer<Layer = DomainLayer>,
    RightSpecification: Layer<Layer = DomainLayer>,
{
    type Layer = DomainLayer;
}

impl<Candidate, LeftSpecification, RightSpecification> Specification<Candidate>
    for Disjunction<LeftSpecification, RightSpecification>
where
    LeftSpecification: Specification<Candidate>,
    RightSpecification: Specification<Candidate>,
{
    fn decide_acceptance(&self, candidate: &Candidate) -> CandidateDecision {
        let Self {
            left_specification,
            right_specification,
        } = self;
        let left_decision = left_specification.decide_acceptance(candidate);
        let right_decision = right_specification.decide_acceptance(candidate);
        left_decision | right_decision
    }
}

impl<LeftSpecification, RightSpecification, NextSpecification> BitAnd<NextSpecification>
    for Disjunction<LeftSpecification, RightSpecification>
{
    type Output = Conjunction<Self, NextSpecification>;

    fn bitand(self, next_specification: NextSpecification) -> Self::Output {
        Conjunction::from((self, next_specification))
    }
}

impl<LeftSpecification, RightSpecification, NextSpecification> BitOr<NextSpecification>
    for Disjunction<LeftSpecification, RightSpecification>
{
    type Output = Disjunction<Self, NextSpecification>;

    fn bitor(self, next_specification: NextSpecification) -> Self::Output {
        Disjunction::from((self, next_specification))
    }
}

impl<LeftSpecification, RightSpecification> Not
    for Disjunction<LeftSpecification, RightSpecification>
{
    type Output = Negation<Self>;

    fn not(self) -> Self::Output {
        Negation::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::specification;

    #[test]
    fn a_disjunction_accepts_a_candidate_its_left_specification_accepts() {
        let candidate: u32 = 10;
        let left_specification = specification::tests::fixture::Acceptance;
        let right_specification = specification::tests::fixture::Rejection;
        let disjunction = Disjunction::from((left_specification, right_specification));

        let candidate_decision = disjunction.decide_acceptance(&candidate);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_disjunction_accepts_a_candidate_only_its_right_specification_accepts() {
        let candidate: u32 = 0;
        let left_specification = specification::tests::fixture::Rejection;
        let right_specification = specification::tests::fixture::Acceptance;
        let disjunction = Disjunction::from((left_specification, right_specification));

        let candidate_decision = disjunction.decide_acceptance(&candidate);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_disjunction_rejects_a_candidate_neither_of_its_specifications_accepts() {
        let candidate: u32 = 5;
        let left_specification = specification::tests::fixture::Rejection;
        let right_specification = specification::tests::fixture::Rejection;
        let disjunction = Disjunction::from((left_specification, right_specification));

        let candidate_decision = disjunction.decide_acceptance(&candidate);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_disjunction_and_a_specification_are_the_conjunction_of_both() {
        let left_specification = specification::tests::fixture::Acceptance;
        let right_specification = specification::tests::fixture::Rejection;
        let next_specification = specification::tests::fixture::Rejection;
        let disjunction = Disjunction::from((left_specification, right_specification));

        let specification = disjunction & next_specification;

        let expectation = Conjunction::from((disjunction, next_specification));
        assert_eq!(specification, expectation);
    }

    #[test]
    fn a_disjunction_or_a_specification_is_the_disjunction_of_both() {
        let left_specification = specification::tests::fixture::Acceptance;
        let right_specification = specification::tests::fixture::Rejection;
        let next_specification = specification::tests::fixture::Rejection;
        let disjunction = Disjunction::from((left_specification, right_specification));

        let specification = disjunction | next_specification;

        let expectation = Disjunction::from((disjunction, next_specification));
        assert_eq!(specification, expectation);
    }

    #[test]
    fn the_negation_of_a_disjunction_is_the_negation_of_it() {
        let left_specification = specification::tests::fixture::Acceptance;
        let right_specification = specification::tests::fixture::Rejection;
        let disjunction = Disjunction::from((left_specification, right_specification));

        let specification = !disjunction;

        let expectation = Negation::from(disjunction);
        assert_eq!(specification, expectation);
    }
}
