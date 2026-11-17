use crate::domain::DomainEvent;
use std::future::Future;

pub trait EventBus<Event: DomainEvent> {
    type Error;

    fn try_publish_event(
        &self,
        event: Event,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
