use crate::layer::DomainLayer;
use crate::layer::Layer;

pub trait Factory<Product: Layer<Layer = DomainLayer>>: Layer<Layer = DomainLayer> {
    type Blueprint<'blueprint>;
    type Error;

    fn try_create_product(&self, blueprint: Self::Blueprint<'_>) -> Result<Product, Self::Error>;
}
