use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub enum Stream {
    Capture,
    #[default]
    Inheritance,
}

impl Layer for Stream {
    type Layer = DomainLayer;
}

impl ValueObject for Stream {}
