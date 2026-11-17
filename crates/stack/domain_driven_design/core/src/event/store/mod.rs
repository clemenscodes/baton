mod events;

use crate::domain::DomainEvent;
use std::future::Future;

pub use events::Events;

pub trait EventStore<Event: DomainEvent + Clone> {
    type Error;

    fn try_append_event(
        &self,
        event: &Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn try_load_events(&self) -> impl Future<Output = Result<Events<Event>, Self::Error>> + Send;
}
