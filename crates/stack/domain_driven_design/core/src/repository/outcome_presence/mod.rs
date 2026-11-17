use crate::layer::DomainLayer;
use crate::layer::Layer;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum OutcomePresence<Outcome> {
    Presence { outcome: Outcome },
    Absence,
}

impl<Outcome> Layer for OutcomePresence<Outcome> {
    type Layer = DomainLayer;
}
