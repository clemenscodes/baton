use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;
use std::string::FromUtf8Error;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Error {
    LibraryDirectoryAbsence,
    LibraryDirectoryEncoding,
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl From<FromUtf8Error> for Error {
    fn from(_: FromUtf8Error) -> Self {
        Self::LibraryDirectoryEncoding
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LibraryDirectoryAbsence => write!(
                formatter,
                "[COVERAGE] rustc names no library directory, so the tools of LLVM beside it cannot be found."
            ),
            Self::LibraryDirectoryEncoding => write!(
                formatter,
                "[COVERAGE] rustc names its library directory in a text that is no UTF-8."
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_library_directory_absence_says_that_the_tools_cannot_be_found() {
        let error = Error::LibraryDirectoryAbsence;

        let text = error.to_string();

        let expectation = "[COVERAGE] rustc names no library directory, so the tools of LLVM beside it cannot be found.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_library_directory_encoding_says_that_the_text_is_no_utf_8() {
        let error = Error::LibraryDirectoryEncoding;

        let text = error.to_string();

        let expectation = "[COVERAGE] rustc names its library directory in a text that is no UTF-8.";
        assert_eq!(text, expectation);
    }
}
