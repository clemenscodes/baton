mod declarations;
mod identifiers;

pub mod tokens;

use crate::code::declarations::Declarations;
use crate::code::identifiers::Identifiers;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Code {
    identifiers: Identifiers,
    declarations: Declarations,
}

impl Layer for Code {
    type Layer = DomainLayer;
}

impl ValueObject for Code {}

impl Code {
    pub fn identifiers(&self) -> &Identifiers {
        match self {
            Self { identifiers, .. } => identifiers,
        }
    }

    pub fn declarations(&self) -> &Declarations {
        match self {
            Self { declarations, .. } => declarations,
        }
    }
}

impl From<(Identifiers, Declarations)> for Code {
    fn from((identifiers, declarations): (Identifiers, Declarations)) -> Self {
        Self {
            identifiers,
            declarations,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::fixture;

    #[test]
    fn a_code_lends_the_identifiers_it_names() {
        let identifier_texts: &[&str] = &["source"];
        let source = fixture::identifier_source_factory(identifier_texts);
        let code = source.production_code();

        let identifiers = code.identifiers();

        let identifier_count = identifiers.into_iter().count();
        let expectation: usize = 1;
        assert_eq!(identifier_count, expectation);
    }

    #[test]
    fn a_code_lends_the_identifiers_it_declares() {
        let declaration_texts: &[&str] = &["source"];
        let source = fixture::declaration_source_factory(declaration_texts);
        let code = source.production_code();

        let declarations = code.declarations();

        let declaration_count = declarations.into_iter().count();
        let expectation: usize = 1;
        assert_eq!(declaration_count, expectation);
    }
}
