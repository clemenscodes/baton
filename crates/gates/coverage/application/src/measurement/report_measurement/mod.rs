use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ReportMeasurement {
    report_path: PathBuf,
    compilation_words: Vec<OsString>,
    test_words: Vec<OsString>,
}

impl Layer for ReportMeasurement {
    type Layer = ApplicationLayer;
}

impl ReportMeasurement {
    const SEPARATOR_WORD: &'static str = "--";

    const PROFILE_ROOT_NAME: &'static str = "profiles";

    pub(crate) fn report_path(&self) -> &Path {
        &self.report_path
    }

    pub(crate) fn compilation_words(&self) -> &[OsString] {
        &self.compilation_words
    }

    pub(crate) fn test_words(&self) -> &[OsString] {
        &self.test_words
    }

    pub(crate) fn compose_profile_root(&self) -> PathBuf {
        self.report_path.with_file_name(Self::PROFILE_ROOT_NAME)
    }
}

impl TryFrom<&[OsString]> for ReportMeasurement {
    type Error = Error;

    fn try_from(words: &[OsString]) -> Result<Self, Self::Error> {
        let report_word = words.first().ok_or(Error::Words)?;
        let command_words = &words[1..];
        let separator_word_criterion = |word: &OsString| *word == *Self::SEPARATOR_WORD;
        let separator_word_index = command_words
            .iter()
            .position(separator_word_criterion)
            .ok_or(Error::Words)?;
        let report_path = PathBuf::from(report_word);
        let compilation_words = command_words[..separator_word_index].to_vec();
        let test_words = command_words[separator_word_index + 1..].to_vec();
        let report_measurement = Self {
            report_path,
            compilation_words,
            test_words,
        };
        Ok(report_measurement)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_words_name_the_report_the_build_and_after_the_separator_the_tests() {
        let word_texts = [
            "target/moon/coverage/harbor.json",
            "cargo",
            "test",
            "--no-run",
            "--",
            "cargo",
            "nextest",
            "run",
        ];
        let words = Vec::from_iter(word_texts.map(OsString::from));

        let report_measurement = ReportMeasurement::try_from(words.as_slice()).unwrap();

        let report_expectation = Path::new("target/moon/coverage/harbor.json");
        let compilation_expectation = Vec::from_iter(
            ["cargo", "test", "--no-run"].map(OsString::from),
        );
        let test_expectation = Vec::from_iter(["cargo", "nextest", "run"].map(OsString::from));
        assert_eq!(report_measurement.report_path(), report_expectation);
        assert_eq!(
            report_measurement.compilation_words(),
            compilation_expectation
        );
        assert_eq!(report_measurement.test_words(), test_expectation);
    }

    #[test]
    fn the_profiles_lie_beside_the_report() {
        let word_texts = ["target/moon/coverage/harbor.json", "--"];
        let words = Vec::from_iter(word_texts.map(OsString::from));
        let report_measurement = ReportMeasurement::try_from(words.as_slice()).unwrap();

        let profile_root = report_measurement.compose_profile_root();

        let expectation = PathBuf::from("target/moon/coverage/profiles");
        assert_eq!(profile_root, expectation);
    }

    #[test]
    fn no_words_are_an_error() {
        let words = Vec::new();

        let report_measurement = ReportMeasurement::try_from(words.as_slice());

        assert!(matches!(report_measurement, Err(Error::Words)));
    }

    #[test]
    fn words_without_the_separator_are_an_error() {
        let word_texts = ["target/moon/coverage/harbor.json", "cargo", "test"];
        let words = Vec::from_iter(word_texts.map(OsString::from));

        let report_measurement = ReportMeasurement::try_from(words.as_slice());

        assert!(matches!(report_measurement, Err(Error::Words)));
    }
}
