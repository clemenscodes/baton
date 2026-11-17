use ::syn::Lit;
use ::syn::ext::IdentExt;
use gates_vocabulary_language_core_language::occurrence::Occurrence;
use gates_vocabulary_language_core_language::vocabulary::word;
use proc_macro2::{Ident, LineColumn, Literal};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct IdentifierToken {
    text: String,
    start_position: LineColumn,
}

impl Layer for IdentifierToken {
    type Layer = InfrastructureLayer;
}

impl IdentifierToken {
    pub(crate) fn text(&self) -> &str {
        match self {
            Self { text, .. } => text,
        }
    }

    pub(crate) fn start(&self) -> LineColumn {
        match self {
            Self { start_position, .. } => *start_position,
        }
    }

    pub(crate) fn line(&self) -> usize {
        match self {
            Self { start_position, .. } => start_position.line,
        }
    }

    pub(crate) fn column(&self) -> usize {
        match self {
            Self { start_position, .. } => start_position.column + 1,
        }
    }
}

impl From<Ident> for IdentifierToken {
    fn from(identifier: Ident) -> Self {
        let span = identifier.span();
        let start_position = span.start();
        let text = identifier.unraw().to_string();
        Self {
            text,
            start_position,
        }
    }
}

impl From<Literal> for IdentifierToken {
    fn from(literal: Literal) -> Self {
        let span = literal.span();
        let start_position = span.start();
        let syntax_literal = Lit::new(literal);
        let suffix = syntax_literal.suffix();
        let text = suffix.to_string();
        Self {
            text,
            start_position,
        }
    }
}

impl TryFrom<&IdentifierToken> for Occurrence {
    type Error = word::Error;

    fn try_from(identifier_token: &IdentifierToken) -> Result<Self, Self::Error> {
        let text = identifier_token.text();
        let line = identifier_token.line();
        let column = identifier_token.column();
        Self::try_from((text, line, column))
    }
}
