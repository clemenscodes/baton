use crate::identifier::Identifier;
use crate::layer::DomainLayer;
use crate::layer::Layer;

pub trait Entity: Layer<Layer = DomainLayer> {
    type Identity: Identifier;

    fn identity(&self) -> &Self::Identity;
}
