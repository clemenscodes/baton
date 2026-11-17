mod candidate_decision;
mod conjunction;
mod disjunction;
mod negation;

use crate::layer::DomainLayer;
use crate::layer::Layer;

pub use candidate_decision::CandidateDecision;
pub use conjunction::Conjunction;
pub use disjunction::Disjunction;
pub use negation::Negation;

pub trait Specification<Candidate>: Layer<Layer = DomainLayer> {
    fn decide_acceptance(&self, candidate: &Candidate) -> CandidateDecision;
}

#[cfg(test)]
mod tests {
    pub(super) mod fixture {
        use crate::layer::DomainLayer;
        use crate::layer::Layer;
        use crate::specification::CandidateDecision;
        use crate::specification::Specification;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Acceptance;

        impl Layer for Acceptance {
            type Layer = DomainLayer;
        }

        impl Specification<u32> for Acceptance {
            fn decide_acceptance(&self, _: &u32) -> CandidateDecision {
                CandidateDecision::Acceptance
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Rejection;

        impl Layer for Rejection {
            type Layer = DomainLayer;
        }

        impl Specification<u32> for Rejection {
            fn decide_acceptance(&self, _: &u32) -> CandidateDecision {
                CandidateDecision::Rejection
            }
        }
    }
}
