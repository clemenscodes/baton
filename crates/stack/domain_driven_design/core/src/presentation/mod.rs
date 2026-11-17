use crate::layer::Layer;
use crate::layer::PresentationLayer;
use crate::model::Model;

pub trait Presentation: Layer<Layer = PresentationLayer> {
    type Model: Model;
}
