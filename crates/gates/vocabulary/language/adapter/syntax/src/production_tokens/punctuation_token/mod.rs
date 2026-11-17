use gates_vocabulary_language_core_language::code::tokens::code_token::CodeToken;
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct PunctuationToken {
    start_position: LineColumn,
    text: String,
}

impl Layer for PunctuationToken {
    type Layer = InfrastructureLayer;
}

impl From<(LineColumn, String)> for PunctuationToken {
    fn from((start_position, text): (LineColumn, String)) -> Self {
        Self {
            start_position,
            text,
        }
    }
}

impl From<PunctuationToken> for CodeToken {
    fn from(punctuation_token: PunctuationToken) -> Self {
        let PunctuationToken {
            start_position,
            text,
        } = punctuation_token;
        let line = start_position.line;
        let column = start_position.column + 1;
        Self::Punctuation { line, column, text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_punctuation_token_turns_into_the_punctuation_at_its_line_and_its_column() {
        let line: usize = 3;
        let column_offset: usize = 4;
        let start_position = LineColumn {
            line,
            column: column_offset,
        };
        let token_text = String::from("{");
        let text = token_text.clone();
        let punctuation_token = PunctuationToken::from((start_position, token_text));

        let code_token = CodeToken::from(punctuation_token);

        let column: usize = 5;
        let expectation = CodeToken::Punctuation { line, column, text };
        assert_eq!(code_token, expectation);
    }
}
