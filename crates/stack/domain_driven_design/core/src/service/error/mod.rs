use crate::layer::ApplicationLayer;
use crate::layer::Layer;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result as FormatResult;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Error<Identity, RepositoryError> {
    Absence { identity: Identity },
    Repository { error: RepositoryError },
}

impl<Identity, RepositoryError> From<RepositoryError> for Error<Identity, RepositoryError> {
    fn from(error: RepositoryError) -> Self {
        Self::Repository { error }
    }
}

impl<Identity, RepositoryError> Layer for Error<Identity, RepositoryError> {
    type Layer = ApplicationLayer;
}

impl<Identity, RepositoryError> Display for Error<Identity, RepositoryError>
where
    Identity: Display,
    RepositoryError: Display,
{
    fn fmt(&self, formatter: &mut Formatter) -> FormatResult {
        match self {
            Self::Absence { identity } => {
                write!(
                    formatter,
                    "the repository has no aggregate with the identity {identity}"
                )
            }
            Self::Repository { error } => {
                write!(formatter, "the repository failed: {error}")
            }
        }
    }
}

impl<Identity, RepositoryError> std::error::Error for Error<Identity, RepositoryError>
where
    Identity: Debug + Display,
    RepositoryError: Debug + Display,
{
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absence_names_the_identity_the_repository_does_not_keep() {
        let identity: u32 = 7;
        let error: Error<u32, &str> = Error::Absence { identity };

        let error_display = error.to_string();

        let expectation = "the repository has no aggregate with the identity 7";
        assert_eq!(error_display, expectation);
    }

    #[test]
    fn an_error_of_the_repository_shows_the_error_it_carries() {
        let repository_error = "the disk is full";
        let error: Error<u32, &str> = Error::Repository {
            error: repository_error,
        };

        let error_display = error.to_string();

        let expectation = "the repository failed: the disk is full";
        assert_eq!(error_display, expectation);
    }

    #[test]
    fn the_error_of_a_repository_converts_into_a_service_error_that_is_an_error() {
        let repository_error = "the disk is full";

        let error: Error<u32, &str> = Error::from(repository_error);

        let expectation = Error::Repository {
            error: repository_error,
        };
        assert_eq!(error, expectation);
    }
}
