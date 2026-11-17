use crate::domain::DomainEvent;
use crate::layer::DomainLayer;
use crate::layer::Layer;

pub trait Policy<Event: DomainEvent>: Layer<Layer = DomainLayer> {
    type Reaction;

    fn compute_reaction(&self, event: &Event) -> Self::Reaction;
}
