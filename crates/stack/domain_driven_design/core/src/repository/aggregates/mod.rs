use crate::layer::DomainLayer;
use crate::layer::Layer;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Aggregates<Aggregate> {
    aggregates: Vec<Aggregate>,
}

impl<Aggregate> Default for Aggregates<Aggregate> {
    fn default() -> Self {
        let aggregates = Vec::new();
        Self { aggregates }
    }
}

impl<Aggregate> FromIterator<Aggregate> for Aggregates<Aggregate> {
    fn from_iter<AggregatesType: IntoIterator<Item = Aggregate>>(
        aggregate_sequence: AggregatesType,
    ) -> Self {
        let aggregates = aggregate_sequence.into_iter().collect();
        Self { aggregates }
    }
}

impl<Aggregate> IntoIterator for Aggregates<Aggregate> {
    type Item = Aggregate;
    type IntoIter = IntoIter<Aggregate>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { aggregates } = self;
        aggregates.into_iter()
    }
}

impl<Aggregate> Layer for Aggregates<Aggregate> {
    type Layer = DomainLayer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aggregates_yield_the_sequence_they_are_collected_from_in_its_order() {
        let expectation: [u32; 2] = [3, 5];
        let aggregates = Aggregates::from_iter(expectation);

        let aggregate_sequence = Vec::from_iter(aggregates);

        assert_eq!(aggregate_sequence, expectation);
    }

    #[test]
    fn the_default_aggregates_yield_no_aggregate() {
        let aggregates: Aggregates<u32> = Aggregates::default();

        let aggregate_sequence = Vec::from_iter(aggregates);

        let expectation: Vec<u32> = Vec::new();
        assert_eq!(aggregate_sequence, expectation);
    }
}
