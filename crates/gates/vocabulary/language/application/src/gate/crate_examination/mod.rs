use gates_vocabulary_language_core_language::Language;
use gates_vocabulary_language_core_language::identifier::Identifier;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct CrateExamination {
    package_identifier: Identifier,
    production_language: Language,
    test_language: Language,
}

impl Layer for CrateExamination {
    type Layer = ApplicationLayer;
}

impl CrateExamination {
    pub(super) fn package_identifier(&self) -> &Identifier {
        &self.package_identifier
    }

    pub(super) fn production_language(&self) -> &Language {
        &self.production_language
    }

    pub(super) fn test_language(&self) -> &Language {
        &self.test_language
    }
}

impl From<(Identifier, Language, Language)> for CrateExamination {
    fn from(
        (package_identifier, production_language, test_language): (Identifier, Language, Language),
    ) -> Self {
        Self {
            package_identifier,
            production_language,
            test_language,
        }
    }
}
