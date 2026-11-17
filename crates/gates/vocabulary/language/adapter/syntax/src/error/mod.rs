use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Error {
    Syntax {
        line: usize,
        column: usize,
        error: ::syn::Error,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax {
                line,
                column,
                error,
            } => write!(
                formatter,
                "{line}:{column}: the source is not valid Rust: {error}"
            ),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Syntax { error, .. } => Some(error),
        }
    }
}

impl From<::syn::Error> for Error {
    fn from(error: ::syn::Error) -> Self {
        let span = error.span();
        let LineColumn {
            line,
            column: column_offset,
        } = span.start();
        let column = column_offset + 1;
        Self::Syntax {
            line,
            column,
            error,
        }
    }
}

impl From<proc_macro2::LexError> for Error {
    fn from(token_error: proc_macro2::LexError) -> Self {
        let error = ::syn::Error::from(token_error);
        Self::from(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn the_source_of_a_syntax_error_is_the_error_of_syn() {
        let error_text = "expected item";
        let span = proc_macro2::Span::call_site();
        let syntax_error = ::syn::Error::new(span, error_text);
        let error = Error::from(syntax_error);

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "expected item";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }
}
