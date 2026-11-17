use gates_vocabulary_language_core_language::source::SourceTokens;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TokenReport {
    source_tokens: SourceTokens,
}

impl Layer for TokenReport {
    type Layer = ApplicationLayer;
}

impl From<SourceTokens> for TokenReport {
    fn from(source_tokens: SourceTokens) -> Self {
        Self { source_tokens }
    }
}

impl fmt::Display for TokenReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { source_tokens } = self;
        write!(formatter, "{source_tokens}")
    }
}
