use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CodeToken {
    Declaration {
        line: usize,
        column: usize,
        text: String,
    },
    Identifier {
        line: usize,
        column: usize,
        text: String,
    },
    Literal {
        line: usize,
        column: usize,
        text: String,
    },
    Punctuation {
        line: usize,
        column: usize,
        text: String,
    },
}

impl Layer for CodeToken {
    type Layer = DomainLayer;
}

impl ValueObject for CodeToken {}

impl fmt::Display for CodeToken {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Declaration { line, column, text } => {
                write!(formatter, "{line}\t{column}\tdeclaration\t{text}")
            }
            Self::Identifier { line, column, text } => {
                write!(formatter, "{line}\t{column}\tidentifier\t{text}")
            }
            Self::Literal { line, column, text } => {
                write!(formatter, "{line}\t{column}\tliteral\t{text}")
            }
            Self::Punctuation { line, column, text } => {
                write!(formatter, "{line}\t{column}\tpunctuation\t{text}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_declaration_states_its_line_its_column_and_its_text() {
        let line: usize = 2;
        let column: usize = 5;
        let text = String::from("anchor");
        let code_token = CodeToken::Declaration { line, column, text };

        let token_text = code_token.to_string();

        let expectation = "2\t5\tdeclaration\tanchor";
        assert_eq!(token_text, expectation);
    }

    #[test]
    fn an_identifier_states_its_line_its_column_and_its_text() {
        let line: usize = 2;
        let column: usize = 5;
        let text = String::from("anchor");
        let code_token = CodeToken::Identifier { line, column, text };

        let token_text = code_token.to_string();

        let expectation = "2\t5\tidentifier\tanchor";
        assert_eq!(token_text, expectation);
    }

    #[test]
    fn a_literal_states_its_line_its_column_and_its_text() {
        let line: usize = 2;
        let column: usize = 5;
        let text = String::from("0x1F");
        let code_token = CodeToken::Literal { line, column, text };

        let token_text = code_token.to_string();

        let expectation = "2\t5\tliteral\t0x1F";
        assert_eq!(token_text, expectation);
    }

    #[test]
    fn a_punctuation_states_its_line_its_column_and_its_text() {
        let line: usize = 2;
        let column: usize = 5;
        let text = String::from("{");
        let code_token = CodeToken::Punctuation { line, column, text };

        let token_text = code_token.to_string();

        let expectation = "2\t5\tpunctuation\t{";
        assert_eq!(token_text, expectation);
    }
}
