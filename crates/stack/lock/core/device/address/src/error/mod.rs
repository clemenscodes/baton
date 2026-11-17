use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    AddressText { text: String },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AddressText { text } => write!(
                formatter,
                "`{text}` is no PCI address of a device, which reads like `0000:03:00.0`."
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_of_an_address_reads_as_a_sentence() {
        let error = Error::AddressText {
            text: String::from("03:00.0"),
        };

        let text = error.to_string();

        assert_eq!(
            text,
            "`03:00.0` is no PCI address of a device, which reads like `0000:03:00.0`."
        );
    }
}
