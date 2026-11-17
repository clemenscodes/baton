use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum DirectoryPresence {
    Presence,
    Absence,
}

impl Layer for DirectoryPresence {
    type Layer = DomainLayer;
}

impl ValueObject for DirectoryPresence {}
