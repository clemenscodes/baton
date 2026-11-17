use crate::Error;
use crate::measurement::ReportMeasurement;
use gates_coverage_core_invocation::{Invocation, Stream, Variable};
use gates_coverage_core_profile_directory::ProfileDirectory;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::ffi::OsString;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct Assignment {
    report_measurement: ReportMeasurement,
    profile_directory: ProfileDirectory,
}

impl Layer for Assignment {
    type Layer = ApplicationLayer;
}

impl Assignment {
    const INSTRUMENTATION_VARIABLE_NAME: &'static str = "CARGO_BUILD_RUSTFLAGS";

    const INSTRUMENTATION_FLAG: &'static str = "-Cinstrument-coverage";

    pub(super) fn profile_directory(&self) -> &ProfileDirectory {
        &self.profile_directory
    }

    pub(super) fn report_path(&self) -> &Path {
        self.report_measurement.report_path()
    }

    pub(super) fn try_compose_compilation_invocation(&self) -> Result<Invocation, Error> {
        let compilation_words = self.report_measurement.compilation_words().to_vec();
        let variables = self.compose_variables();
        let stream = Stream::Capture;
        let invocation = Invocation::try_from((compilation_words, variables, stream))?;
        Ok(invocation)
    }

    pub(super) fn try_compose_test_invocation(&self) -> Result<Invocation, Error> {
        let test_words = self.report_measurement.test_words().to_vec();
        let variables = self.compose_variables();
        let stream = Stream::Inheritance;
        let invocation = Invocation::try_from((test_words, variables, stream))?;
        Ok(invocation)
    }

    fn compose_variables(&self) -> Vec<Variable> {
        let profile_variable = self.profile_directory.compose_profile_variable();
        let name = OsString::from(Self::INSTRUMENTATION_VARIABLE_NAME);
        let value = OsString::from(Self::INSTRUMENTATION_FLAG);
        let instrumentation_variable = Variable::from((name, value));
        vec![profile_variable, instrumentation_variable]
    }
}

impl From<(ReportMeasurement, ProfileDirectory)> for Assignment {
    fn from(
        (report_measurement, profile_directory): (ReportMeasurement, ProfileDirectory),
    ) -> Self {
        Self {
            report_measurement,
            profile_directory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    mod fixture {
        use super::*;

        pub(crate) fn assignment_factory(word_texts: &[&str]) -> Assignment {
            let words = Vec::from_iter(word_texts.iter().map(OsString::from));
            let report_measurement = ReportMeasurement::try_from(words.as_slice()).unwrap();
            let path = PathBuf::from("/tmp/coverage-7");
            let profile_directory = ProfileDirectory::from(path);
            Assignment::from((report_measurement, profile_directory))
        }

        pub(crate) fn variables_factory() -> Vec<Variable> {
            let profile_name = OsString::from("LLVM_PROFILE_FILE");
            let profile_value = OsString::from("/tmp/coverage-7/%p-%m.profraw");
            let profile_variable = Variable::from((profile_name, profile_value));
            let instrumentation_name = OsString::from("CARGO_BUILD_RUSTFLAGS");
            let instrumentation_value = OsString::from("-Cinstrument-coverage");
            let instrumentation_variable = Variable::from((
                instrumentation_name,
                instrumentation_value,
            ));
            vec![profile_variable, instrumentation_variable]
        }
    }

    #[test]
    fn an_assignment_keeps_its_profile_directory_and_the_path_of_its_report() {
        let word_texts = ["target/moon/coverage/harbor.json", "--"];

        let assignment = fixture::assignment_factory(&word_texts);

        let profile_directory_expectation = ProfileDirectory::from(PathBuf::from(
            "/tmp/coverage-7",
        ));
        let report_expectation = Path::new("target/moon/coverage/harbor.json");
        assert_eq!(
            assignment.profile_directory(),
            &profile_directory_expectation
        );
        assert_eq!(assignment.report_path(), report_expectation);
    }

    #[test]
    fn the_build_captures_its_output_and_writes_its_profiles_into_the_directory() {
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
        let assignment = fixture::assignment_factory(&word_texts);

        let invocation = assignment.try_compose_compilation_invocation().unwrap();

        let program = OsString::from("cargo");
        let arguments = Vec::from_iter(["test", "--no-run"].map(OsString::from));
        let variables = fixture::variables_factory();
        let stream = Stream::Capture;
        let expectation = Invocation::from((program, arguments, variables, stream));
        assert_eq!(invocation, expectation);
    }

    #[test]
    fn the_tests_show_their_output_and_write_their_profiles_into_the_directory() {
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
        let assignment = fixture::assignment_factory(&word_texts);

        let invocation = assignment.try_compose_test_invocation().unwrap();

        let program = OsString::from("cargo");
        let arguments = Vec::from_iter(["nextest", "run"].map(OsString::from));
        let variables = fixture::variables_factory();
        let stream = Stream::Inheritance;
        let expectation = Invocation::from((program, arguments, variables, stream));
        assert_eq!(invocation, expectation);
    }

    #[test]
    fn a_build_without_a_program_is_an_error() {
        let word_texts = ["target/moon/coverage/harbor.json", "--", "cargo"];
        let assignment = fixture::assignment_factory(&word_texts);

        let invocation = assignment.try_compose_compilation_invocation();

        assert!(matches!(invocation, Err(Error::Invocation { .. })));
    }

    #[test]
    fn tests_without_a_program_are_an_error() {
        let word_texts = ["target/moon/coverage/harbor.json", "cargo", "--"];
        let assignment = fixture::assignment_factory(&word_texts);

        let invocation = assignment.try_compose_test_invocation();

        assert!(matches!(invocation, Err(Error::Invocation { .. })));
    }
}
