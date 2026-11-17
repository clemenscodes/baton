use crate::layer::seal::Seal;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct InfrastructureLayer;

impl Seal for InfrastructureLayer {}
