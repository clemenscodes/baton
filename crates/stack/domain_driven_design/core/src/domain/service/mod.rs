use crate::layer::DomainLayer;
use crate::layer::Layer;

pub trait DomainService: Layer<Layer = DomainLayer> {}
