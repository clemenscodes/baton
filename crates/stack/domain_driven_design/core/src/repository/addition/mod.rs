use crate::layer::DomainLayer;
use crate::layer::Layer;
use crate::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Addition {
    Insertion,
    Collision,
}

impl Layer for Addition {
    type Layer = DomainLayer;
}

impl ValueObject for Addition {}
