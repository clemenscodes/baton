use crate::layer::InfrastructureLayer;
use crate::layer::Layer;

pub trait Adapter: Layer<Layer = InfrastructureLayer> {}
