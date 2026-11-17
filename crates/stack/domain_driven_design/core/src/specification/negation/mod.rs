use crate::layer::DomainLayer;
use crate::layer::Layer;
use crate::specification::CandidateDecision;
use crate::specification::Conjunction;
use crate::specification::Disjunction;
use crate::specification::Specification;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Not;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Negation<InnerSpecification> {
    inner_specification: InnerSpecification,
}

impl<InnerSpecification> From<InnerSpecification> for Negation<InnerSpecification> {
    fn from(inner_specification: InnerSpecification) -> Self {
        Self {
            inner_specification,
        }
    }
}

impl<InnerSpecification> Layer for Negation<InnerSpecification>
where
    InnerSpecification: Layer<Layer = DomainLayer>,
{
    type Layer = DomainLayer;
}

impl<Candidate, InnerSpecification> Specification<Candidate> for Negation<InnerSpecification>
where
    InnerSpecification: Specification<Candidate>,
{
    fn decide_acceptance(&self, candidate: &Candidate) -> CandidateDecision {
        let Self {
            inner_specification,
        } = self;
        let inner_decision = inner_specification.decide_acceptance(candidate);
        !inner_decision
    }
}

impl<InnerSpecification, NextSpecification> BitAnd<NextSpecification>
    for Negation<InnerSpecification>
{
    type Output = Conjunction<Self, NextSpecification>;

    fn bitand(self, next_specification: NextSpecification) -> Self::Output {
        Conjunction::from((self, next_specification))
    }
}

impl<InnerSpecification, NextSpecification> BitOr<NextSpecification>
    for Negation<InnerSpecification>
{
    type Output = Disjunction<Self, NextSpecification>;

    fn bitor(self, next_specification: NextSpecification) -> Self::Output {
        Disjunction::from((self, next_specification))
    }
}

impl<InnerSpecification> Not for Negation<InnerSpecification> {
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
    fn a_negation_accepts_a_candidate_its_inner_specification_rejects() {
        let candidate: u32 = 5;
        let inner_specification = specification::tests::fixture::Rejection;
        let negation = Negation::from(inner_specification);

        let candidate_decision = negation.decide_acceptance(&candidate);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_negation_rejects_a_candidate_its_inner_specification_accepts() {
        let candidate: u32 = 15;
        let inner_specification = specification::tests::fixture::Acceptance;
        let negation = Negation::from(inner_specification);

        let candidate_decision = negation.decide_acceptance(&candidate);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(candidate_decision, expectation);
    }

    #[test]
    fn a_negation_and_a_specification_are_the_conjunction_of_both() {
        let inner_specification = specification::tests::fixture::Rejection;
        let next_specification = specification::tests::fixture::Acceptance;
        let negation = Negation::from(inner_specification);

        let specification = negation & next_specification;

        let expectation = Conjunction::from((negation, next_specification));
        assert_eq!(specification, expectation);
    }

    #[test]
    fn a_negation_or_a_specification_is_the_disjunction_of_both() {
        let inner_specification = specification::tests::fixture::Rejection;
        let next_specification = specification::tests::fixture::Acceptance;
        let negation = Negation::from(inner_specification);

        let specification = negation | next_specification;

        let expectation = Disjunction::from((negation, next_specification));
        assert_eq!(specification, expectation);
    }

    #[test]
    fn the_negation_of_a_negation_is_the_negation_of_it() {
        let inner_specification = specification::tests::fixture::Rejection;
        let negation = Negation::from(inner_specification);

        let specification = !negation;

        let expectation = Negation::from(negation);
        assert_eq!(specification, expectation);
    }
}
