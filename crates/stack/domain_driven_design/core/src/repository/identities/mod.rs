use crate::layer::DomainLayer;
use crate::layer::Layer;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identities<Identity> {
    identities: Vec<Identity>,
}

impl<Identity> Default for Identities<Identity> {
    fn default() -> Self {
        let identities = Vec::new();
        Self { identities }
    }
}

impl<Identity> FromIterator<Identity> for Identities<Identity> {
    fn from_iter<IdentitiesType: IntoIterator<Item = Identity>>(
        identity_sequence: IdentitiesType,
    ) -> Self {
        let identities = identity_sequence.into_iter().collect();
        Self { identities }
    }
}

impl<'identities, Identity> IntoIterator for &'identities Identities<Identity> {
    type Item = &'identities Identity;
    type IntoIter = Iter<'identities, Identity>;

    fn into_iter(self) -> Self::IntoIter {
        let Identities { identities } = self;
        identities.iter()
    }
}

impl<Identity> Layer for Identities<Identity> {
    type Layer = DomainLayer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identities_yield_the_sequence_they_are_collected_from_in_its_order() {
        let identifier_sequence: [u32; 2] = [5, 3];
        let identities = Identities::from_iter(identifier_sequence);

        let identity_sequence = Vec::from_iter(&identities);

        let expectation = Vec::from_iter(&identifier_sequence);
        assert_eq!(identity_sequence, expectation);
    }

    #[test]
    fn the_default_identities_yield_no_identity() {
        let identities: Identities<u32> = Identities::default();

        let identity_sequence = Vec::from_iter(&identities);

        let expectation: Vec<&u32> = Vec::new();
        assert_eq!(identity_sequence, expectation);
    }
}
