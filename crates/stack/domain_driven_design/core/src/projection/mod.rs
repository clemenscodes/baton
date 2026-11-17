use crate::domain::DomainEvent;
use crate::read_model::ReadModel;

pub trait Projection<Event: DomainEvent>: ReadModel
    + Sized
    + for<'event> From<(Self, &'event Event)>
{
}
