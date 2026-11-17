use crate::layer::ApplicationLayer;
use crate::layer::Layer;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Change<Aggregate, Outcome> {
    aggregate: Aggregate,
    outcome: Outcome,
}

impl<Aggregate, Outcome> From<(Aggregate, Outcome)> for Change<Aggregate, Outcome> {
    fn from((aggregate, outcome): (Aggregate, Outcome)) -> Self {
        Self { aggregate, outcome }
    }
}

impl<Aggregate, Outcome> Layer for Change<Aggregate, Outcome> {
    type Layer = ApplicationLayer;
}

impl<Aggregate, Outcome> Change<Aggregate, Outcome> {
    pub fn aggregate(&self) -> &Aggregate {
        &self.aggregate
    }

    pub fn outcome(self) -> Outcome {
        self.outcome
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_change_keeps_the_aggregate_it_is_made_of() {
        let expectation: u32 = 3;
        let outcome = "the counter rose";
        let change = Change::from((expectation, outcome));

        let aggregate = *change.aggregate();

        assert_eq!(aggregate, expectation);
    }

    #[test]
    fn a_change_keeps_the_outcome_it_is_made_of() {
        let aggregate: u32 = 3;
        let outcome = "the counter rose";
        let change = Change::from((aggregate, outcome));

        let command_outcome = change.outcome();

        assert_eq!(command_outcome, outcome);
    }
}
