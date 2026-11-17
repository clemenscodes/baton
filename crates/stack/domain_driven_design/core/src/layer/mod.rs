mod application_layer;
mod domain_layer;
mod infrastructure_layer;
mod presentation_layer;
mod seal;

use crate::layer::seal::Seal;

pub use application_layer::ApplicationLayer;
pub use domain_layer::DomainLayer;
pub use infrastructure_layer::InfrastructureLayer;
pub use presentation_layer::PresentationLayer;

pub trait Layer {
    type Layer: Seal;
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(super) struct DomainEvent;

        impl Layer for DomainEvent {
            type Layer = DomainLayer;
        }
    }

    #[test]
    fn a_type_of_the_domain_layer_names_the_domain_layer_as_its_layer() {
        let expectation = DomainLayer;

        let layer: DomainLayer = <fixture::DomainEvent as Layer>::Layer::default();

        assert_eq!(layer, expectation);
    }
}
