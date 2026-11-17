use crate::layer::ApplicationLayer;
use crate::layer::Layer;

pub trait ApplicationService: Layer<Layer = ApplicationLayer> {}
