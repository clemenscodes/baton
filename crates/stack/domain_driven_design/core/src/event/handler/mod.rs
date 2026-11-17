use crate::domain::DomainEvent;
use crate::layer::ApplicationLayer;
use crate::layer::Layer;
use std::future::Future;

pub trait EventHandler<Event: DomainEvent>: Layer<Layer = ApplicationLayer> {
    type Outcome;

    fn receive_event(&self, event: &Event) -> impl Future<Output = Self::Outcome> + Send;
}
