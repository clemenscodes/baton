use crate::layer::seal::Seal;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct DomainLayer;

impl Seal for DomainLayer {}
