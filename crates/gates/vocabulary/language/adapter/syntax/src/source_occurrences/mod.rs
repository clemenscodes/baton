use crate::attribute_visitor::AttributeVisitor;
use crate::code_occurrences::CodeOccurrences;
use crate::declaration_visitor::DeclarationVisitor;
use crate::identifier_tokens::identifier_token::IdentifierToken;
use gates_vocabulary_language_core_language::Code;
use gates_vocabulary_language_core_language::code::tokens::CodeTokens;
use gates_vocabulary_language_core_language::defects::defect::Defect;
use gates_vocabulary_language_core_language::source::Source;
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct SourceOccurrences<'syntax> {
    production_end: LineColumn,
    comment_starts: &'syntax BTreeSet<LineColumn>,
    test_function_starts: &'syntax BTreeSet<LineColumn>,
    production_occurrences: CodeOccurrences<'syntax>,
    test_occurrences: CodeOccurrences<'syntax>,
    defect_list: Vec<Defect>,
    code_tokens: CodeTokens,
}

impl Layer for SourceOccurrences<'_> {
    type Layer = InfrastructureLayer;
}

impl SourceOccurrences<'_> {
    pub(super) fn insert_token(&mut self, identifier_token: &IdentifierToken) {
        let start_position = identifier_token.start();
        if self.comment_starts.contains(&start_position)
            || self.test_function_starts.contains(&start_position)
        {
            return;
        }
        let code_occurrences = match start_position < self.production_end {
            true => &mut self.production_occurrences,
            false => &mut self.test_occurrences,
        };
        let insertion = code_occurrences.try_insert_token(identifier_token);
        if let Err(error) = insertion {
            let line = identifier_token.line();
            let column = identifier_token.column();
            let text = identifier_token.text().to_string();
            let defect = Defect::Identifier {
                line,
                column,
                text,
                error,
            };
            self.defect_list.push(defect);
        }
    }
}

impl<'syntax>
    From<(
        LineColumn,
        &'syntax AttributeVisitor,
        &'syntax DeclarationVisitor,
        CodeTokens,
    )> for SourceOccurrences<'syntax>
{
    fn from(
        (production_end, attribute_visitor, declaration_visitor, code_tokens): (
            LineColumn,
            &'syntax AttributeVisitor,
            &'syntax DeclarationVisitor,
            CodeTokens,
        ),
    ) -> Self {
        let declaration_starts = declaration_visitor.starts();
        let comment_starts = attribute_visitor.comment_starts();
        let test_function_starts = declaration_visitor.test_function_starts();
        let production_occurrences = CodeOccurrences::from(declaration_starts);
        let test_occurrences = CodeOccurrences::from(declaration_starts);
        let defect_list = attribute_visitor.defects().to_vec();
        Self {
            production_end,
            comment_starts,
            test_function_starts,
            production_occurrences,
            test_occurrences,
            defect_list,
            code_tokens,
        }
    }
}

impl From<SourceOccurrences<'_>> for Source {
    fn from(source_occurrences: SourceOccurrences<'_>) -> Self {
        let SourceOccurrences {
            production_occurrences,
            test_occurrences,
            defect_list,
            code_tokens,
            ..
        } = source_occurrences;
        let production_code = Code::from(production_occurrences);
        let test_code = Code::from(test_occurrences);
        let defects = defect_list.into_iter().collect();
        Self::from((production_code, code_tokens, test_code, defects))
    }
}
