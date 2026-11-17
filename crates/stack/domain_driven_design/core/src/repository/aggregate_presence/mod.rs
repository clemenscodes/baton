use crate::layer::DomainLayer;
use crate::layer::Layer;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum AggregatePresence<Aggregate> {
    Presence { aggregate: Aggregate },
    Absence,
}

impl<Aggregate> From<Option<Aggregate>> for AggregatePresence<Aggregate> {
    fn from(stored_aggregate: Option<Aggregate>) -> Self {
        match stored_aggregate {
            Some(aggregate) => Self::Presence { aggregate },
            None => Self::Absence,
        }
    }
}

impl<Aggregate> Layer for AggregatePresence<Aggregate> {
    type Layer = DomainLayer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_stored_aggregate_converts_into_its_presence() {
        let aggregate: u32 = 3;
        let stored_aggregate = Some(aggregate);

        let aggregate_presence = AggregatePresence::from(stored_aggregate);

        let presence = AggregatePresence::Presence { aggregate };
        assert_eq!(aggregate_presence, presence);
    }

    #[test]
    fn no_stored_aggregate_converts_into_an_absence() {
        let stored_aggregate: Option<u32> = None;

        let aggregate_presence = AggregatePresence::from(stored_aggregate);

        let absence = AggregatePresence::Absence;
        assert_eq!(aggregate_presence, absence);
    }
}
