pub mod code_token;

use crate::code::tokens::code_token::CodeToken;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CodeTokens {
    code_tokens: Vec<CodeToken>,
}

impl Layer for CodeTokens {
    type Layer = DomainLayer;
}

impl ValueObject for CodeTokens {}

impl FromIterator<CodeToken> for CodeTokens {
    fn from_iter<CodeTokensType: IntoIterator<Item = CodeToken>>(
        code_token_sequence: CodeTokensType,
    ) -> Self {
        let code_tokens = code_token_sequence.into_iter().collect();
        Self { code_tokens }
    }
}

impl<'code_tokens> IntoIterator for &'code_tokens CodeTokens {
    type Item = &'code_tokens CodeToken;
    type IntoIter = Iter<'code_tokens, CodeToken>;

    fn into_iter(self) -> Self::IntoIter {
        let CodeTokens { code_tokens } = self;
        code_tokens.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_tokens_keep_the_order_of_their_sequence() {
        let line: usize = 1;
        let first_column: usize = 2;
        let first_text = String::from("anchor");
        let first_code_token = CodeToken::Identifier {
            line,
            column: first_column,
            text: first_text,
        };
        let last_column: usize = 1;
        let last_text = String::from("=");
        let last_code_token = CodeToken::Punctuation {
            line,
            column: last_column,
            text: last_text,
        };
        let code_token_list = [first_code_token, last_code_token];

        let code_tokens: CodeTokens = code_token_list.into_iter().collect();

        let token_texts: Vec<String> = code_tokens.into_iter().map(ToString::to_string).collect();
        let expectation = ["1\t2\tidentifier\tanchor", "1\t1\tpunctuation\t="];
        assert_eq!(token_texts, expectation);
    }
}
