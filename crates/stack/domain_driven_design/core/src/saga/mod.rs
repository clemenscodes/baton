use crate::layer::ApplicationLayer;
use crate::layer::Layer;

pub trait Saga: Layer<Layer = ApplicationLayer> {}
