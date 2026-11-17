use crate::code::tokens::CodeTokens;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SourceTokens {
    path_tokens: BTreeMap<PathBuf, CodeTokens>,
}

impl Layer for SourceTokens {
    type Layer = DomainLayer;
}

impl ValueObject for SourceTokens {}

impl From<BTreeMap<PathBuf, CodeTokens>> for SourceTokens {
    fn from(path_tokens: BTreeMap<PathBuf, CodeTokens>) -> Self {
        Self { path_tokens }
    }
}

impl fmt::Display for SourceTokens {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { path_tokens } = self;
        for (path, code_tokens) in path_tokens {
            let path_text = path.display();
            for code_token in code_tokens {
                writeln!(formatter, "{path_text}\t{code_token}")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::code::tokens::code_token::CodeToken;
    use std::io::{ErrorKind, Write};

    #[test]
    fn source_tokens_state_every_token_with_the_path_of_its_source() {
        let line: usize = 1;
        let column: usize = 4;
        let text = String::from("anchor");
        let code_token = CodeToken::Declaration { line, column, text };
        let code_token_list = [code_token];
        let code_tokens: CodeTokens = code_token_list.into_iter().collect();
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let path_token_entry = (path, code_tokens);
        let path_token_list = [path_token_entry];
        let path_tokens = BTreeMap::from(path_token_list);
        let source_tokens = SourceTokens::from(path_tokens);

        let token_text = source_tokens.to_string();

        let expectation = "crates/harbor/src/lib.rs\t1\t4\tdeclaration\tanchor\n";
        assert_eq!(token_text, expectation);
    }

    #[test]
    fn source_tokens_hand_on_the_error_of_a_full_buffer() {
        let line: usize = 1;
        let column: usize = 4;
        let text = String::from("anchor");
        let code_token = CodeToken::Declaration { line, column, text };
        let code_token_list = [code_token];
        let code_tokens: CodeTokens = code_token_list.into_iter().collect();
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let path_token_entry = (path, code_tokens);
        let path_token_list = [path_token_entry];
        let path_tokens = BTreeMap::from(path_token_list);
        let source_tokens = SourceTokens::from(path_tokens);
        let mut token_bytes: &mut [u8] = &mut [];

        let outcome = write!(token_bytes, "{source_tokens}");

        let error_kind = outcome.map_err(|error| error.kind());
        let expectation = Err(ErrorKind::WriteZero);
        assert_eq!(error_kind, expectation);
    }
}
