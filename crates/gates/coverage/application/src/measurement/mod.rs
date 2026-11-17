mod assignment;
mod report_measurement;

pub use report_measurement::ReportMeasurement;

use crate::Error;
use crate::measurement::assignment::Assignment;
use gates_coverage_core_compiler::Compiler;
use gates_coverage_core_invocation::Completion;
use gates_coverage_core_invocation::Invocation;
use gates_coverage_core_invocation::Spawner;
use gates_coverage_core_mapping_presence::MappingPresence;
use gates_coverage_core_profile_directory::ProfileStore;
use gates_coverage_core_profile_tools::ProfileTools;
use gates_coverage_core_report::Decoder;
use gates_coverage_core_report::ReportFile;
use gates_coverage_core_test_build::TestBuild;
use stack_domain_driven_design_core::application_service::ApplicationService;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Measurement<SpawnerType, ProfileStoreType, DecoderType> {
    spawner: SpawnerType,
    profile_store: ProfileStoreType,
    decoder: DecoderType,
    compiler: Compiler,
}

impl<SpawnerType, ProfileStoreType, DecoderType> Layer
    for Measurement<SpawnerType, ProfileStoreType, DecoderType>
{
    type Layer = ApplicationLayer;
}

impl<SpawnerType, ProfileStoreType, DecoderType> ApplicationService
    for Measurement<SpawnerType, ProfileStoreType, DecoderType>
{
}

impl<SpawnerType, ProfileStoreType, DecoderType> From<(SpawnerType, ProfileStoreType, DecoderType)>
    for Measurement<SpawnerType, ProfileStoreType, DecoderType>
{
    fn from(
        (spawner, profile_store, decoder): (SpawnerType, ProfileStoreType, DecoderType),
    ) -> Self {
        let compiler = Compiler;
        Self {
            spawner,
            profile_store,
            decoder,
            compiler,
        }
    }
}

impl<SpawnerType: Spawner, ProfileStoreType: ProfileStore, DecoderType: Decoder>
    SynchronousCommandHandler<ReportMeasurement>
    for Measurement<SpawnerType, ProfileStoreType, DecoderType>
{
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, report_measurement: ReportMeasurement) -> Self::Outcome {
        self.try_measure_report(report_measurement)
    }
}

impl<SpawnerType: Spawner, ProfileStoreType: ProfileStore, DecoderType: Decoder>
    Measurement<SpawnerType, ProfileStoreType, DecoderType>
{
    fn try_measure_report(&self, report_measurement: ReportMeasurement) -> Result<(), Error> {
        let profile_root = report_measurement.compose_profile_root();
        let profile_directory = self
            .profile_store
            .try_create_profile_directory(&profile_root)
            .map_err(|error| Error::ProfileDirectory { error })?;
        let assignment = Assignment::from((report_measurement, profile_directory));
        let report_file = self.try_measure_assignment(&assignment);
        let removal = self
            .profile_store
            .try_remove_profile_directory(assignment.profile_directory());
        let report_file = report_file?;
        removal.map_err(|error| Error::ProfileDirectory { error })?;
        self.try_write_report_file(&report_file)
    }

    fn try_measure_assignment(&self, assignment: &Assignment) -> Result<ReportFile, Error> {
        let test_build = self.try_compile_tests(assignment)?;
        let test_invocation = assignment.try_compose_test_invocation()?;
        self.try_spawn(&test_invocation)?;
        let profile_tools = self.try_locate_profile_tools(assignment)?;
        let profile_paths = self.try_list_profiles(assignment)?;
        let inspection_invocation = profile_tools.compose_inspection_invocation(&test_build);
        let inspection_output = self.try_spawn(&inspection_invocation)?;
        let mapping_presence = MappingPresence::from(inspection_output.as_slice());
        let export = match mapping_presence {
            MappingPresence::Presence => {
                let merge_invocation = profile_tools.compose_merge_invocation(&profile_paths);
                self.try_spawn(&merge_invocation)?;
                let export_invocation = profile_tools.compose_export_invocation(&test_build);
                self.try_spawn(&export_invocation)?
            }
            MappingPresence::Absence => Vec::from(ProfileTools::EMPTY_EXPORT),
        };
        let report_path = assignment.report_path().to_path_buf();
        let report_file = ReportFile::from((report_path, export));
        Ok(report_file)
    }

    fn try_compile_tests(&self, assignment: &Assignment) -> Result<TestBuild, Error> {
        let compilation_invocation = assignment.try_compose_compilation_invocation()?;
        let compilation_output = self.try_spawn(&compilation_invocation)?;
        self.decoder
            .try_decode_test_build(&compilation_output)
            .map_err(|error| {
                let text = error.to_string();
                Error::TestBuild { text }
            })
    }

    fn try_locate_profile_tools(&self, assignment: &Assignment) -> Result<ProfileTools, Error> {
        let library_invocation = self.compiler.compose_library_invocation();
        let library_output = self.try_spawn(&library_invocation)?;
        let profile_directory = assignment.profile_directory();
        let profile_data_path = profile_directory.compose_profile_data_path();
        let profile_tools = ProfileTools::try_from((library_output, profile_data_path))?;
        Ok(profile_tools)
    }

    fn try_list_profiles(&self, assignment: &Assignment) -> Result<Vec<PathBuf>, Error> {
        let profile_directory = assignment.profile_directory();
        let profile_paths = self
            .profile_store
            .try_list_profiles(profile_directory)
            .map_err(|error| Error::ProfileDirectory { error })?;
        if profile_paths.is_empty() {
            return Err(Error::ProfileAbsence);
        }
        Ok(profile_paths)
    }

    fn try_spawn(&self, invocation: &Invocation) -> Result<Vec<u8>, Error> {
        let program = invocation.program().to_os_string();
        match self.spawner.try_spawn_invocation(invocation) {
            Ok(Completion::Success { output }) => Ok(output),
            Ok(Completion::Failure { status }) => Err(Error::Failure { program, status }),
            Err(error) => Err(Error::ProgramStart { program, error }),
        }
    }

    fn try_write_report_file(&self, report_file: &ReportFile) -> Result<(), Error> {
        self.profile_store
            .try_write_report_file(report_file)
            .map_err(|error| {
                let path = report_file.path().to_path_buf();
                Error::ReportFile { path, error }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Outcome;
    use gates_coverage_core_invocation::{Stream, Variable};
    use gates_coverage_core_profile_directory::ProfileDirectory;
    use gates_coverage_core_report::Report;
    use stack_domain_driven_design_core::adapter::Adapter;
    use stack_domain_driven_design_core::layer::InfrastructureLayer;
    use stack_domain_driven_design_core::port::{DrivenPort, Port};
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::env::consts::EXE_SUFFIX;
    use std::ffi::OsString;
    use std::fmt;
    use std::io::{self, Write};
    use std::path::Path;
    use std::rc::Rc;

    #[derive(Debug, Default)]
    struct SpawnerAdapter {
        completions: RefCell<VecDeque<Result<Completion, io::ErrorKind>>>,
        invocations: Rc<RefCell<Vec<Invocation>>>,
    }

    impl Layer for SpawnerAdapter {
        type Layer = InfrastructureLayer;
    }

    impl Adapter for SpawnerAdapter {}

    impl Port for SpawnerAdapter {}

    impl DrivenPort for SpawnerAdapter {}

    impl Spawner for SpawnerAdapter {
        fn try_spawn_invocation(&self, invocation: &Invocation) -> io::Result<Completion> {
            self.invocations.borrow_mut().push(invocation.clone());
            let completion = self.completions.borrow_mut().pop_front().unwrap();
            completion.map_err(io::Error::from)
        }
    }

    #[derive(Debug)]
    struct ProfileStoreAdapter {
        profile_directory_result: Result<ProfileDirectory, io::ErrorKind>,
        profile_paths_result: Result<Vec<PathBuf>, io::ErrorKind>,
        removal: Result<(), io::ErrorKind>,
        report_file_result: Result<(), io::ErrorKind>,
        profile_directory_removals: Rc<RefCell<Vec<ProfileDirectory>>>,
        report_files: Rc<RefCell<Vec<ReportFile>>>,
    }

    impl Layer for ProfileStoreAdapter {
        type Layer = InfrastructureLayer;
    }

    impl Adapter for ProfileStoreAdapter {}

    impl Port for ProfileStoreAdapter {}

    impl DrivenPort for ProfileStoreAdapter {}

    impl ProfileStore for ProfileStoreAdapter {
        fn try_create_profile_directory(&self, _: &Path) -> io::Result<ProfileDirectory> {
            self.profile_directory_result
                .clone()
                .map_err(io::Error::from)
        }

        fn try_list_profiles(&self, _: &ProfileDirectory) -> io::Result<Vec<PathBuf>> {
            self.profile_paths_result.clone().map_err(io::Error::from)
        }

        fn try_remove_profile_directory(
            &self,
            profile_directory: &ProfileDirectory,
        ) -> io::Result<()> {
            let profile_directory_removal = profile_directory.clone();
            self.profile_directory_removals
                .borrow_mut()
                .push(profile_directory_removal);
            self.removal.map_err(io::Error::from)
        }

        fn try_write_report_file(&self, report_file: &ReportFile) -> io::Result<()> {
            self.report_files.borrow_mut().push(report_file.clone());
            self.report_file_result.map_err(io::Error::from)
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct DecoderError;

    impl fmt::Display for DecoderError {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                formatter,
                "[COVERAGE] the build made no executable of a test."
            )
        }
    }

    #[derive(Debug)]
    struct DecoderAdapter {
        test_build: Result<TestBuild, DecoderError>,
    }

    impl Layer for DecoderAdapter {
        type Layer = InfrastructureLayer;
    }

    impl Adapter for DecoderAdapter {}

    impl Port for DecoderAdapter {}

    impl DrivenPort for DecoderAdapter {}

    impl Decoder for DecoderAdapter {
        type Error = DecoderError;

        fn try_decode_report(&self, _: impl io::Read) -> Result<Report, Self::Error> {
            Err(DecoderError)
        }

        fn try_decode_test_build(&self, _: &[u8]) -> Result<TestBuild, Self::Error> {
            let test_build = self.test_build.as_ref();
            test_build.cloned().map_err(|_| DecoderError)
        }
    }

    mod fixture {
        use super::*;

        pub(crate) const LIBRARY_TEXT: &str = "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n";

        pub(crate) fn failure_factory() -> Result<Completion, io::ErrorKind> {
            let status = String::from("exit status: 1");
            Ok(Completion::Failure { status })
        }

        pub(crate) fn failure_text_factory(tool_name: &str) -> String {
            let program_text = tool_word_factory(tool_name);
            let program_display = program_text.display();
            format!("[COVERAGE] `{program_display}` ended with exit status: 1.\n")
        }

        pub(crate) const WORD_TEXTS: [&str; 7] = [
            "target/moon/coverage/harbor.json",
            "cargo",
            "test",
            "--no-run",
            "--",
            "cargo",
            "nextest",
        ];

        pub(crate) fn words_factory() -> Vec<OsString> {
            Vec::from_iter(WORD_TEXTS.map(OsString::from))
        }

        pub(crate) fn profile_directory_factory() -> ProfileDirectory {
            let path = PathBuf::from("/tmp/coverage-7");
            ProfileDirectory::from(path)
        }

        pub(crate) fn test_build_factory() -> TestBuild {
            let executable = PathBuf::from("/target/debug/deps/harbor-0a1b");
            let executables = vec![executable];
            let manifest_path = PathBuf::from("/workspace/crates/harbor/Cargo.toml");
            TestBuild::from((executables, manifest_path))
        }

        pub(crate) fn profile_store_factory() -> ProfileStoreAdapter {
            let profile_path = PathBuf::from("/tmp/coverage-7/11-22.profraw");
            ProfileStoreAdapter {
                profile_directory_result: Ok(profile_directory_factory()),
                profile_paths_result: Ok(vec![profile_path]),
                removal: Ok(()),
                report_file_result: Ok(()),
                profile_directory_removals: Rc::default(),
                report_files: Rc::default(),
            }
        }

        pub(crate) fn success_factory(output_text: &str) -> Result<Completion, io::ErrorKind> {
            let output = output_text.as_bytes().to_vec();
            Ok(Completion::Success { output })
        }

        pub(crate) fn spawner_factory(
            completions: Vec<Result<Completion, io::ErrorKind>>,
        ) -> SpawnerAdapter {
            let completions = RefCell::new(VecDeque::from(completions));
            let invocations = Rc::default();
            SpawnerAdapter {
                completions,
                invocations,
            }
        }

        pub(crate) fn full_spawner_factory() -> SpawnerAdapter {
            let library_text = "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n";
            spawner_factory(vec![
                success_factory("build"),
                success_factory(""),
                success_factory(library_text),
                success_factory(MAPPING_SECTION_INSPECTION_TEXT),
                success_factory(""),
                success_factory("{\"data\":[]}"),
            ])
        }

        pub(crate) const MAPPING_SECTION_INSPECTION_TEXT: &str = r#"/target/debug/deps/harbor-0a1b:	file format elf64-x86-64
Sections:
Idx Name               Size     VMA              Type
 44 __llvm_covmap      0001f844 0000000000000000
"#;

        pub(crate) fn inspection_invocation_factory() -> Invocation {
            Invocation::from((
                tool_word_factory("llvm-objdump"),
                Vec::from_iter(
                    ["--section-headers", "/target/debug/deps/harbor-0a1b"].map(OsString::from),
                ),
                Vec::new(),
                Stream::Capture,
            ))
        }

        pub(crate) fn decoder_factory() -> DecoderAdapter {
            let test_build = Ok(test_build_factory());
            DecoderAdapter { test_build }
        }

        pub(crate) fn measurement_outcome(
            spawner: SpawnerAdapter,
            profile_store: ProfileStoreAdapter,
            decoder: DecoderAdapter,
            words: Vec<OsString>,
            errors: &mut Vec<u8>,
        ) -> Outcome {
            let measurement = Measurement::from((spawner, profile_store, decoder));
            let report_measurement = ReportMeasurement::try_from(words.as_slice());
            let measurement_outcome = |command: ReportMeasurement| {
                measurement.dispatch_command(command)
            };
            match report_measurement.and_then(measurement_outcome) {
                Ok(()) => Outcome::Success,
                Err(error) => {
                    writeln!(errors, "{error}").unwrap();
                    Outcome::Error
                }
            }
        }

        pub(crate) fn tool_word_factory(tool_name: &str) -> OsString {
            let tool_path_text = format!(
                "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin/{tool_name}{EXE_SUFFIX}"
            );
            OsString::from(tool_path_text)
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
    fn a_measurement_builds_runs_merges_and_exports_the_tests_and_writes_their_report() {
        let spawner = fixture::full_spawner_factory();
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let invocations = Rc::clone(&spawner.invocations);
        let profile_directory_removals = Rc::clone(&profile_store.profile_directory_removals);
        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let compilation_invocation = Invocation::from((
            OsString::from("cargo"),
            Vec::from_iter(["test", "--no-run"].map(OsString::from)),
            fixture::variables_factory(),
            Stream::Capture,
        ));
        let test_invocation = Invocation::from((
            OsString::from("cargo"),
            Vec::from_iter(["nextest"].map(OsString::from)),
            fixture::variables_factory(),
            Stream::Inheritance,
        ));
        let library_invocation = Compiler.compose_library_invocation();
        let merge_invocation = Invocation::from((
            fixture::tool_word_factory("llvm-profdata"),
            Vec::from_iter(
                [
                    "merge",
                    "-sparse",
                    "/tmp/coverage-7/11-22.profraw",
                    "-o",
                    "/tmp/coverage-7/coverage.profdata",
                ]
                .map(OsString::from),
            ),
            Vec::new(),
            Stream::Inheritance,
        ));
        let export_invocation = Invocation::from((
            fixture::tool_word_factory("llvm-cov"),
            Vec::from_iter(
                [
                    "export",
                    "-format=text",
                    "-instr-profile=/tmp/coverage-7/coverage.profdata",
                    "/target/debug/deps/harbor-0a1b",
                    "/workspace/crates/harbor/src",
                ]
                .map(OsString::from),
            ),
            Vec::new(),
            Stream::Capture,
        ));
        let inspection_invocation = fixture::inspection_invocation_factory();
        let invocations_expectation = vec![
            compilation_invocation,
            test_invocation,
            library_invocation,
            inspection_invocation,
            merge_invocation,
            export_invocation,
        ];
        let report_path = PathBuf::from("target/moon/coverage/harbor.json");
        let export = b"{\"data\":[]}".to_vec();
        let report_file_expectation = ReportFile::from((report_path, export));
        let removal_expectation = fixture::profile_directory_factory();
        assert_eq!(outcome, Outcome::Success);
        assert_eq!(error_text, "");
        assert_eq!(*invocations.borrow(), invocations_expectation);
        assert_eq!(*report_files.borrow(), [report_file_expectation]);
        assert_eq!(*profile_directory_removals.borrow(), [removal_expectation]);
    }

    #[test]
    fn tests_whose_executables_carry_no_coverage_mapping_leave_a_report_without_a_region() {
        let library_text = "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n";
        let inspection_text = r#"/target/debug/deps/harbor-0a1b:	file format elf64-x86-64
Sections:
Idx Name               Size     VMA              Type
 17 .text              0010b272 00000000000e2a80 TEXT
"#;
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory(library_text),
            fixture::success_factory(inspection_text),
        ]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let invocations = Rc::clone(&spawner.invocations);
        let profile_directory_removals = Rc::clone(&profile_store.profile_directory_removals);
        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let last_invocation = invocations.borrow().last().cloned();
        let inspection_invocation = fixture::inspection_invocation_factory();
        let invocation_expectation = Some(inspection_invocation);
        let report_path = PathBuf::from("target/moon/coverage/harbor.json");
        let export = br#"{"data": []}"#.to_vec();
        let report_file_expectation = ReportFile::from((report_path, export));
        let removal_expectation = fixture::profile_directory_factory();
        assert_eq!(outcome, Outcome::Success);
        assert_eq!(error_text, "");
        assert_eq!(invocations.borrow().len(), 4);
        assert_eq!(last_invocation, invocation_expectation);
        assert_eq!(*report_files.borrow(), [report_file_expectation]);
        assert_eq!(*profile_directory_removals.borrow(), [removal_expectation]);
    }

    #[test]
    fn an_inspection_of_the_executables_that_fails_is_an_error() {
        let library_text = "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n";
        let status = String::from("exit status: 1");
        let failure = Ok(Completion::Failure { status });
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory(library_text),
            failure,
        ]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let program_text = fixture::tool_word_factory("llvm-objdump");
        let program_display = program_text.display();
        let expectation = format!("[COVERAGE] `{program_display}` ended with exit status: 1.\n");
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
        assert_eq!(*report_files.borrow(), []);
    }

    #[test]
    fn words_without_the_separator_are_an_error() {
        let spawner = fixture::spawner_factory(Vec::new());
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = vec![OsString::from("target/moon/coverage/harbor.json")];

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] a measurement takes the path of its report, the command of the build, `--` and the command of the tests.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_profile_directory_that_cannot_be_made_is_an_error() {
        let spawner = fixture::spawner_factory(Vec::new());
        let mut profile_store = fixture::profile_store_factory();
        profile_store.profile_directory_result = Err(io::ErrorKind::PermissionDenied);
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] cannot keep the profiles of the measurement in a directory of their own: permission denied.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn tests_that_fail_are_an_error_and_their_profile_directory_is_removed() {
        let status = String::from("exit status: 100");
        let failure = Ok(Completion::Failure { status });
        let spawner = fixture::spawner_factory(vec![fixture::success_factory("build"), failure]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let profile_directory_removals = Rc::clone(&profile_store.profile_directory_removals);
        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let removal_expectation = fixture::profile_directory_factory();
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(
            error_text,
            "[COVERAGE] `cargo` ended with exit status: 100.\n"
        );
        assert_eq!(*profile_directory_removals.borrow(), [removal_expectation]);
        assert_eq!(*report_files.borrow(), []);
    }

    #[test]
    fn a_build_that_cannot_start_is_an_error() {
        let spawner = fixture::spawner_factory(vec![Err(io::ErrorKind::NotFound)]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        assert_eq!(outcome, Outcome::Error);
        assert_eq!(
            error_text,
            "[COVERAGE] cannot start `cargo`: entity not found.\n"
        );
    }

    #[test]
    fn a_build_without_a_test_executable_is_an_error() {
        let spawner = fixture::spawner_factory(vec![fixture::success_factory("build")]);
        let profile_store = fixture::profile_store_factory();
        let test_build = Err(DecoderError);
        let decoder = DecoderAdapter { test_build };
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        assert_eq!(outcome, Outcome::Error);
        assert_eq!(
            error_text,
            "[COVERAGE] the build made no executable of a test.\n"
        );
    }

    #[test]
    fn a_compiler_without_a_library_directory_is_an_error() {
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory("\n"),
        ]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] rustc names no library directory, so the tools of LLVM beside it cannot be found.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn tests_that_leave_no_profile_are_an_error() {
        let library_text = "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n";
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory(library_text),
        ]);
        let mut profile_store = fixture::profile_store_factory();
        profile_store.profile_paths_result = Ok(Vec::new());
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] the tests left no profile, so their build was not instrumented for coverage.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn profiles_that_cannot_be_listed_are_an_error() {
        let library_text = "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n";
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory(library_text),
        ]);
        let mut profile_store = fixture::profile_store_factory();
        profile_store.profile_paths_result = Err(io::ErrorKind::PermissionDenied);
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] cannot keep the profiles of the measurement in a directory of their own: permission denied.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_profile_directory_that_cannot_be_removed_is_an_error() {
        let spawner = fixture::full_spawner_factory();
        let mut profile_store = fixture::profile_store_factory();
        profile_store.removal = Err(io::ErrorKind::PermissionDenied);
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] cannot keep the profiles of the measurement in a directory of their own: permission denied.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
        assert_eq!(*report_files.borrow(), []);
    }

    #[test]
    fn a_report_that_cannot_be_written_is_an_error() {
        let spawner = fixture::full_spawner_factory();
        let mut profile_store = fixture::profile_store_factory();
        profile_store.report_file_result = Err(io::ErrorKind::PermissionDenied);
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] cannot write the report `target/moon/coverage/harbor.json`: permission denied.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_build_without_a_program_is_an_error() {
        let spawner = fixture::spawner_factory(Vec::new());
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let word_texts = ["target/moon/coverage/harbor.json", "--", "cargo"];
        let words = Vec::from_iter(word_texts.map(OsString::from));

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] a command names no program: the build and the tests each need one.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn tests_without_a_program_are_an_error() {
        let spawner = fixture::spawner_factory(vec![fixture::success_factory("build")]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let word_texts = ["target/moon/coverage/harbor.json", "cargo", "--"];
        let words = Vec::from_iter(word_texts.map(OsString::from));

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] a command names no program: the build and the tests each need one.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_compiler_that_fails_to_name_its_library_directory_is_an_error() {
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::failure_factory(),
        ]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = "[COVERAGE] `rustc` ended with exit status: 1.\n";
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_merge_of_the_profiles_that_fails_is_an_error() {
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory(fixture::LIBRARY_TEXT),
            fixture::success_factory(fixture::MAPPING_SECTION_INSPECTION_TEXT),
            fixture::failure_factory(),
        ]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = fixture::failure_text_factory("llvm-profdata");
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
        assert_eq!(*report_files.borrow(), []);
    }

    #[test]
    fn an_export_of_the_profiles_that_fails_is_an_error() {
        let spawner = fixture::spawner_factory(vec![
            fixture::success_factory("build"),
            fixture::success_factory(""),
            fixture::success_factory(fixture::LIBRARY_TEXT),
            fixture::success_factory(fixture::MAPPING_SECTION_INSPECTION_TEXT),
            fixture::success_factory(""),
            fixture::failure_factory(),
        ]);
        let profile_store = fixture::profile_store_factory();
        let decoder = fixture::decoder_factory();
        let words = fixture::words_factory();

        let report_files = Rc::clone(&profile_store.report_files);
        let mut errors = Vec::new();

        let outcome = fixture::measurement_outcome(
            spawner,
            profile_store,
            decoder,
            words,
            &mut errors,
        );

        let error_text = String::from_utf8(errors).unwrap();

        let expectation = fixture::failure_text_factory("llvm-cov");
        assert_eq!(outcome, Outcome::Error);
        assert_eq!(error_text, expectation);
        assert_eq!(*report_files.borrow(), []);
    }
}
