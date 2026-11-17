use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Refresh;

impl Layer for Refresh {
    type Layer = ApplicationLayer;
}
