use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Mode {
    Judgment { report_path: Option<PathBuf> },
    Measurement { words: Vec<OsString> },
}

impl Layer for Mode {
    type Layer = PresentationLayer;
}

impl Mode {
    const MEASUREMENT_WORD: &'static str = "measure";
}

impl From<Vec<OsString>> for Mode {
    fn from(arguments: Vec<OsString>) -> Self {
        let mut argument_sequence = arguments.into_iter();
        let first_argument = argument_sequence.next();
        match first_argument {
            Some(argument) if argument == Self::MEASUREMENT_WORD => {
                let words = Vec::from_iter(argument_sequence);
                Self::Measurement { words }
            }
            first_argument => {
                let report_path = first_argument.map(PathBuf::from);
                Self::Judgment { report_path }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_argument_judges_the_report_of_the_standard_input() {
        let arguments = Vec::new();

        let mode = Mode::from(arguments);

        let report_path = None;
        let expectation = Mode::Judgment { report_path };
        assert_eq!(mode, expectation);
    }

    #[test]
    fn a_path_judges_the_report_it_names() {
        let argument = OsString::from("target/moon/coverage/harbor.json");
        let arguments = vec![argument];

        let mode = Mode::from(arguments);

        let path = PathBuf::from("target/moon/coverage/harbor.json");
        let report_path = Some(path);
        let expectation = Mode::Judgment { report_path };
        assert_eq!(mode, expectation);
    }

    #[test]
    fn the_word_measure_measures_with_the_words_after_it() {
        let mode_word = OsString::from("measure");
        let report_word = OsString::from("target/moon/coverage/harbor.json");
        let arguments = vec![mode_word, report_word];

        let mode = Mode::from(arguments);

        let word = OsString::from("target/moon/coverage/harbor.json");
        let words = vec![word];
        let expectation = Mode::Measurement { words };
        assert_eq!(mode, expectation);
    }
}
