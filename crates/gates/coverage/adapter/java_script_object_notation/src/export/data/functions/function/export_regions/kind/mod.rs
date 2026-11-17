use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) enum Kind {
    #[default]
    Code,
    Annotation,
}

impl Layer for Kind {
    type Layer = InfrastructureLayer;
}

impl From<usize> for Kind {
    fn from(export_kind: usize) -> Self {
        match export_kind {
            0 => Self::Code,
            1.. => Self::Annotation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_export_kind_zero_is_code() {
        let export_kind: usize = 0;

        let kind = Kind::from(export_kind);

        let expectation = Kind::Code;
        assert_eq!(kind, expectation);
    }

    #[test]
    fn every_later_export_kind_is_an_annotation() {
        let export_kind: usize = 4;

        let kind = Kind::from(export_kind);

        let expectation = Kind::Annotation;
        assert_eq!(kind, expectation);
    }

    #[test]
    fn the_default_kind_is_the_kind_of_the_export_kind_zero() {
        let export_kind: usize = 0;

        let kind = Kind::default();

        let expectation = Kind::from(export_kind);
        assert_eq!(kind, expectation);
    }
}
