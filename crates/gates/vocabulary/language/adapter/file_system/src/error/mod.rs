use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Removal {
        replacement_error: io::Error,
        temporary_path: PathBuf,
        error: io::Error,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Removal {
                replacement_error,
                temporary_path,
                error,
            } => {
                let temporary_path_text = temporary_path.display();
                write!(
                    formatter,
                    "{replacement_error}, and the temporary file `{temporary_path_text}` remains because it cannot be removed: {error}"
                )
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Removal { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn a_removal_error_states_both_errors_and_the_path_that_remains() {
        let replacement_error_text = "no room";
        let replacement_error = io::Error::other(replacement_error_text);
        let temporary_path_text = "crates/harbor/mod.rs.7";
        let temporary_path = PathBuf::from(temporary_path_text);
        let removal_error_text = "no right";
        let removal_error = io::Error::other(removal_error_text);
        let error = Error::Removal {
            replacement_error,
            temporary_path,
            error: removal_error,
        };

        let text = error.to_string();

        let expectation = "no room, and the temporary file `crates/harbor/mod.rs.7` remains because it cannot be removed: no right";
        assert_eq!(text, expectation);
    }

    #[test]
    fn the_source_of_a_removal_error_is_the_error_of_the_removal() {
        let replacement_error_text = "no room";
        let replacement_error = io::Error::other(replacement_error_text);
        let temporary_path_text = "crates/harbor/mod.rs.7";
        let temporary_path = PathBuf::from(temporary_path_text);
        let removal_error_text = "no right";
        let removal_error = io::Error::other(removal_error_text);
        let error = Error::Removal {
            replacement_error,
            temporary_path,
            error: removal_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "no right";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }
}
