use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::Query;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct StatusQuery;

impl Layer for StatusQuery {
    type Layer = ApplicationLayer;
}

impl Query for StatusQuery {
    type Output = Result<Vec<String>, Error>;
}
