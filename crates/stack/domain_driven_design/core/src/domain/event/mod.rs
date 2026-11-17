use crate::layer::DomainLayer;
use crate::layer::Layer;
use std::fmt::Debug;

pub trait DomainEvent: Debug + Layer<Layer = DomainLayer> {}
