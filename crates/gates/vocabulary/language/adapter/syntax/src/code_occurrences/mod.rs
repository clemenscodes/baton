use crate::identifier_tokens::identifier_token::IdentifierToken;
use gates_vocabulary_language_core_language::Code;
use gates_vocabulary_language_core_language::occurrence::Occurrence;
use gates_vocabulary_language_core_language::vocabulary::word;
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct CodeOccurrences<'syntax> {
    declaration_starts: &'syntax BTreeSet<LineColumn>,
    identifiers: Vec<Occurrence>,
    declarations: Vec<Occurrence>,
}

impl Layer for CodeOccurrences<'_> {
    type Layer = InfrastructureLayer;
}

impl CodeOccurrences<'_> {
    pub(super) fn try_insert_token(
        &mut self,
        identifier_token: &IdentifierToken,
    ) -> Result<(), word::Error> {
        let Self {
            declaration_starts,
            identifiers,
            declarations,
        } = self;
        let occurrence = Occurrence::try_from(identifier_token)?;
        let start_position = identifier_token.start();
        match declaration_starts.contains(&start_position) {
            true => declarations.push(occurrence),
            false => identifiers.push(occurrence),
        }
        Ok(())
    }
}

impl<'syntax> From<&'syntax BTreeSet<LineColumn>> for CodeOccurrences<'syntax> {
    fn from(declaration_starts: &'syntax BTreeSet<LineColumn>) -> Self {
        let identifiers = Vec::new();
        let declarations = Vec::new();
        Self {
            declaration_starts,
            identifiers,
            declarations,
        }
    }
}

impl From<CodeOccurrences<'_>> for Code {
    fn from(code_occurrences: CodeOccurrences<'_>) -> Self {
        let CodeOccurrences {
            identifiers: identifier_list,
            declarations: declaration_list,
            ..
        } = code_occurrences;
        let identifiers = identifier_list.into_iter().collect();
        let declarations = declaration_list.into_iter().collect();
        Self::from((identifiers, declarations))
    }
}
