use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Group { path: PathBuf, text: String },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Group { path, text } => write!(formatter, "{} failed, {text}", path.display()),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_of_a_group_reads_as_a_phrase() {
        let error = Error::Group {
            path: PathBuf::from("/sys/fs/cgroup/baton"),
            text: String::from("Permission denied"),
        };

        let text = error.to_string();

        assert_eq!(text, "/sys/fs/cgroup/baton failed, Permission denied");
    }
}
