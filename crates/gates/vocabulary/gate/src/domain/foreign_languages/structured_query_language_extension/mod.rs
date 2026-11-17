use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct StructuredQueryLanguageExtension;

impl Layer for StructuredQueryLanguageExtension {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for StructuredQueryLanguageExtension {}

impl StructuredQueryLanguageExtension {
    const NAME: &'static str = "sqlx";

    const NAMES: &'static [&'static str] = &[
        "begin",
        "commit",
        "connect_lazy_with",
        "execute",
        "Executor",
        "fetch_all",
        "fetch_optional",
        "get_database",
        "migrate",
        "MigrateError",
        "PgArguments",
        "PgConnectOptions",
        "PgPool",
        "PgPoolOptions",
        "PgQueryResult",
        "PgRow",
        "Postgres",
        "postgres",
        "Row",
        "rows_affected",
        "run",
        "sqlx",
        "try_get",
    ];
}

impl TryFrom<StructuredQueryLanguageExtension> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: StructuredQueryLanguageExtension) -> Result<Self, Self::Error> {
        let foreign_name_text = StructuredQueryLanguageExtension::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = StructuredQueryLanguageExtension::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_sqlx_is_a_name() {
        let foreign_crate = StructuredQueryLanguageExtension;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
