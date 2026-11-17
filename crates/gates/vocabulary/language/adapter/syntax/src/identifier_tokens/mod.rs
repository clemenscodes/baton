pub(super) mod identifier_token;

use crate::identifier_tokens::identifier_token::IdentifierToken;
use proc_macro2::{TokenStream, TokenTree};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct IdentifierTokens {
    identifier_tokens: Vec<IdentifierToken>,
}

impl Layer for IdentifierTokens {
    type Layer = InfrastructureLayer;
}

impl From<TokenStream> for IdentifierTokens {
    fn from(file_token_stream: TokenStream) -> Self {
        let mut identifier_tokens = Vec::new();
        let mut token_streams = vec![file_token_stream];
        while let Some(token_stream) = token_streams.pop() {
            for token_tree in token_stream {
                match token_tree {
                    TokenTree::Group(group) => {
                        let group_token_stream = group.stream();
                        token_streams.push(group_token_stream);
                    }
                    TokenTree::Ident(identifier) => {
                        let identifier_token = IdentifierToken::from(identifier);
                        identifier_tokens.push(identifier_token);
                    }
                    TokenTree::Literal(literal) => {
                        let identifier_token = IdentifierToken::from(literal);
                        identifier_tokens.push(identifier_token);
                    }
                    TokenTree::Punct(_) => {}
                }
            }
        }
        Self { identifier_tokens }
    }
}

impl<'identifier_tokens> IntoIterator for &'identifier_tokens IdentifierTokens {
    type Item = &'identifier_tokens IdentifierToken;
    type IntoIter = Iter<'identifier_tokens, IdentifierToken>;

    fn into_iter(self) -> Self::IntoIter {
        let IdentifierTokens { identifier_tokens } = self;
        identifier_tokens.iter()
    }
}
