use crate::layer::DomainLayer;
use crate::layer::Layer;

pub trait ValueObject: Clone + Eq + Layer<Layer = DomainLayer> {}
