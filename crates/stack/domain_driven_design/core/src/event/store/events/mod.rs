use crate::layer::DomainLayer;
use crate::layer::Layer;
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Events<Event> {
    events: Vec<Event>,
}

impl<Event> Default for Events<Event> {
    fn default() -> Self {
        let events = Vec::new();
        Self { events }
    }
}

impl<Event> FromIterator<Event> for Events<Event> {
    fn from_iter<EventsType: IntoIterator<Item = Event>>(event_sequence: EventsType) -> Self {
        let events = event_sequence.into_iter().collect();
        Self { events }
    }
}

impl<Event> IntoIterator for Events<Event> {
    type Item = Event;
    type IntoIter = IntoIter<Event>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { events } = self;
        events.into_iter()
    }
}

impl<Event> Layer for Events<Event> {
    type Layer = DomainLayer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn events_yield_the_sequence_they_are_collected_from_in_its_order() {
        let expectation: [u32; 2] = [3, 5];
        let events = Events::from_iter(expectation);

        let event_sequence = Vec::from_iter(events);

        assert_eq!(event_sequence, expectation);
    }

    #[test]
    fn the_default_events_yield_no_event() {
        let events: Events<u32> = Events::default();

        let event_sequence = Vec::from_iter(events);

        let expectation: Vec<u32> = Vec::new();
        assert_eq!(event_sequence, expectation);
    }
}
