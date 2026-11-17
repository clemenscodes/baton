use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Completion {
    Success {
        output: Vec<u8>,
    },
    Failure {
        status: String,
        maybe_code: Option<i32>,
    },
}

impl Layer for Completion {
    type Layer = DomainLayer;
}

impl ValueObject for Completion {}
