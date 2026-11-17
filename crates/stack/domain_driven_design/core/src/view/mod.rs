use crate::layer::Layer;
use crate::layer::PresentationLayer;

pub trait View: Clone + PartialEq + Layer<Layer = PresentationLayer> {}
