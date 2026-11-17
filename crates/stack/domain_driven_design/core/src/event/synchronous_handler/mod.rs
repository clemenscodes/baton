use crate::domain::DomainEvent;
use crate::layer::ApplicationLayer;
use crate::layer::Layer;

pub trait SynchronousEventHandler<Event: DomainEvent>: Layer<Layer = ApplicationLayer> {
    type Outcome;

    fn receive_event(&self, event: &Event) -> Self::Outcome;
}
