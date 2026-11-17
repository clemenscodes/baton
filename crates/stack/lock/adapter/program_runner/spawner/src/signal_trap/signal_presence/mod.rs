use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum SignalPresence {
    Presence,
    Absence,
}

impl Layer for SignalPresence {
    type Layer = InfrastructureLayer;
}
