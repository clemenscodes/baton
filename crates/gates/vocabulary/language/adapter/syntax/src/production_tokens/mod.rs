mod punctuation_token;

use ::syn::ext::IdentExt;
use gates_vocabulary_language_core_language::code::tokens::CodeTokens;
use gates_vocabulary_language_core_language::code::tokens::code_token::CodeToken;
use proc_macro2::{Delimiter, Group, Ident, LineColumn, Literal, Punct, TokenStream, TokenTree};
use punctuation_token::PunctuationToken;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct ProductionTokens<'syntax> {
    production_end: LineColumn,
    declaration_starts: &'syntax BTreeSet<LineColumn>,
    code_tokens: Vec<CodeToken>,
}

impl Layer for ProductionTokens<'_> {
    type Layer = InfrastructureLayer;
}

impl ProductionTokens<'_> {
    pub(super) fn insert_tokens(&mut self, token_stream: TokenStream) {
        for token_tree in token_stream {
            self.insert_token(token_tree);
        }
    }

    fn insert_token(&mut self, token_tree: TokenTree) {
        let span = token_tree.span();
        let start_position = span.start();
        if start_position < self.production_end {
            match token_tree {
                TokenTree::Group(group) => self.insert_token_group(group),
                TokenTree::Ident(identifier) => self.insert_token_identifier(identifier),
                TokenTree::Literal(literal) => self.insert_token_literal(literal),
                TokenTree::Punct(punctuation) => self.insert_token_punctuation(punctuation),
            }
        }
    }

    fn insert_token_group(&mut self, group: Group) {
        let group_texts = match group.delimiter() {
            Delimiter::Parenthesis => ["(", ")"],
            Delimiter::Brace => ["{", "}"],
            Delimiter::Bracket => ["[", "]"],
            Delimiter::None => ["", ""],
        };
        let [start_text, end_text] = group_texts;
        let start_span = group.span_open();
        let start_position = start_span.start();
        let start_token_text = String::from(start_text);
        let start_token = PunctuationToken::from((start_position, start_token_text));
        self.insert_punctuation_token(start_token);
        let token_stream = group.stream();
        self.insert_tokens(token_stream);
        let end_span = group.span_close();
        let end_position = end_span.start();
        let end_token_text = String::from(end_text);
        let end_token = PunctuationToken::from((end_position, end_token_text));
        self.insert_punctuation_token(end_token);
    }

    fn insert_token_identifier(&mut self, identifier: Ident) {
        let Self {
            declaration_starts,
            code_tokens,
            ..
        } = self;
        let span = identifier.span();
        let start_position = span.start();
        let line = start_position.line;
        let column = start_position.column + 1;
        let text = identifier.unraw().to_string();
        let code_token = match declaration_starts.contains(&start_position) {
            true => CodeToken::Declaration { line, column, text },
            false => CodeToken::Identifier { line, column, text },
        };
        code_tokens.push(code_token);
    }

    fn insert_token_literal(&mut self, literal: Literal) {
        let Self { code_tokens, .. } = self;
        let span = literal.span();
        let start_position = span.start();
        let line = start_position.line;
        let column = start_position.column + 1;
        let text = literal.to_string();
        let literal_criterion = |letter: char| letter.is_ascii_digit();
        if text.starts_with(literal_criterion) {
            let code_token = CodeToken::Literal { line, column, text };
            code_tokens.push(code_token);
        }
    }

    fn insert_token_punctuation(&mut self, punctuation: Punct) {
        let span = punctuation.span();
        let start_position = span.start();
        let text = punctuation.as_char().to_string();
        let punctuation_token = PunctuationToken::from((start_position, text));
        self.insert_punctuation_token(punctuation_token);
    }

    fn insert_punctuation_token(&mut self, punctuation_token: PunctuationToken) {
        let Self { code_tokens, .. } = self;
        let code_token = CodeToken::from(punctuation_token);
        code_tokens.push(code_token);
    }
}

impl<'syntax> From<(LineColumn, &'syntax BTreeSet<LineColumn>)> for ProductionTokens<'syntax> {
    fn from(
        (production_end, declaration_starts): (LineColumn, &'syntax BTreeSet<LineColumn>),
    ) -> Self {
        let code_tokens = Vec::new();
        Self {
            production_end,
            declaration_starts,
            code_tokens,
        }
    }
}

impl From<ProductionTokens<'_>> for CodeTokens {
    fn from(production_tokens: ProductionTokens<'_>) -> Self {
        let ProductionTokens { code_tokens, .. } = production_tokens;
        code_tokens.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    mod fixture {
        use super::*;

        pub(super) const TEXT_END: LineColumn = LineColumn {
            line: usize::MAX,
            column: usize::MAX,
        };

        pub(super) fn token_texts_factory(code_tokens: CodeTokens) -> Vec<String> {
            code_tokens.into_iter().map(ToString::to_string).collect()
        }
    }

    #[test]
    fn a_group_of_no_delimiter_states_its_tokens_between_empty_punctuation() {
        let declaration_starts = BTreeSet::new();
        let production_end = fixture::TEXT_END;
        let mut production_tokens = ProductionTokens::from((production_end, &declaration_starts));
        let group_text = "anchor";
        let group_stream = TokenStream::from_str(group_text).unwrap();
        let delimiter = Delimiter::None;
        let group = Group::new(delimiter, group_stream);
        let token_tree = TokenTree::Group(group);
        let token_stream = TokenStream::from(token_tree);

        production_tokens.insert_tokens(token_stream);

        let code_tokens = CodeTokens::from(production_tokens);
        let token_texts = fixture::token_texts_factory(code_tokens);
        let expectation = [
            "1\t1\tpunctuation\t",
            "1\t1\tidentifier\tanchor",
            "1\t1\tpunctuation\t",
        ];
        assert_eq!(token_texts, expectation);
    }

    #[test]
    fn a_token_behind_the_production_end_is_no_production_token() {
        let declaration_starts = BTreeSet::new();
        let line: usize = 2;
        let column: usize = 0;
        let production_end = LineColumn { line, column };
        let mut production_tokens = ProductionTokens::from((production_end, &declaration_starts));
        let text = "anchor\nchain";
        let token_stream = TokenStream::from_str(text).unwrap();

        production_tokens.insert_tokens(token_stream);

        let code_tokens = CodeTokens::from(production_tokens);
        let token_texts = fixture::token_texts_factory(code_tokens);
        let expectation = ["1\t1\tidentifier\tanchor"];
        assert_eq!(token_texts, expectation);
    }
}
