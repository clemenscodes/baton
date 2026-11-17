mod error;

pub use error::Error;

use gates_coverage_core_invocation::{Invocation, Stream};
use gates_coverage_core_test_build::TestBuild;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::env::consts::EXE_SUFFIX;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ProfileTools {
    tool_directory: PathBuf,
    profile_data_path: PathBuf,
}

impl Layer for ProfileTools {
    type Layer = DomainLayer;
}

impl ValueObject for ProfileTools {}

impl ProfileTools {
    const TOOL_DIRECTORY_NAME: &'static str = "bin";

    const MERGE_TOOL_NAME: &'static str = "llvm-profdata";

    const EXPORT_TOOL_NAME: &'static str = "llvm-cov";

    const OBJECT_FLAG: &'static str = "-object";

    const INSPECTION_TOOL_NAME: &'static str = "llvm-objdump";

    const SECTION_HEADER_FLAG: &'static str = "--section-headers";

    pub const EMPTY_EXPORT: &'static [u8] = br#"{"data": []}"#;

    pub fn compose_inspection_invocation(&self, test_build: &TestBuild) -> Invocation {
        let program = self.compose_tool_path(Self::INSPECTION_TOOL_NAME);
        let section_header_flag = OsString::from(Self::SECTION_HEADER_FLAG);
        let executable_words = test_build
            .executables()
            .iter()
            .map(|executable| executable.clone().into_os_string());
        let arguments = Vec::from_iter([section_header_flag].into_iter().chain(executable_words));
        let variables = Vec::new();
        let stream = Stream::Capture;
        Invocation::from((program, arguments, variables, stream))
    }

    pub fn compose_merge_invocation(&self, profile_paths: &[PathBuf]) -> Invocation {
        let program = self.compose_tool_path(Self::MERGE_TOOL_NAME);
        let merge_words = ["merge", "-sparse"].map(OsString::from);
        let profile_data_flag = OsString::from("-o");
        let profile_data_word = self.profile_data_path.clone().into_os_string();
        let profile_words = profile_paths
            .iter()
            .map(|path| path.clone().into_os_string());
        let arguments = Vec::from_iter(
            merge_words
                .into_iter()
                .chain(profile_words)
                .chain([profile_data_flag, profile_data_word]),
        );
        let variables = Vec::new();
        let stream = Stream::Inheritance;
        Invocation::from((program, arguments, variables, stream))
    }

    pub fn compose_export_invocation(&self, test_build: &TestBuild) -> Invocation {
        let program = self.compose_tool_path(Self::EXPORT_TOOL_NAME);
        let profile_data_text = self.profile_data_path.display();
        let profile_data_argument = OsString::from(format!("-instr-profile={profile_data_text}"));
        let export_words = [OsString::from("export"), OsString::from("-format=text")];
        let mut arguments = Vec::from_iter(export_words.into_iter().chain([profile_data_argument]));
        for (index, executable) in test_build.executables().iter().enumerate() {
            if index > 0 {
                arguments.push(OsString::from(Self::OBJECT_FLAG));
            }
            arguments.push(executable.clone().into_os_string());
        }
        let source_directory = test_build.compose_source_directory();
        arguments.push(source_directory.into_os_string());
        let variables = Vec::new();
        let stream = Stream::Capture;
        Invocation::from((program, arguments, variables, stream))
    }

    fn compose_tool_path(&self, tool_name: &str) -> OsString {
        let tool_file_name = format!("{tool_name}{EXE_SUFFIX}");
        let tool_path = self.tool_directory.join(tool_file_name);
        tool_path.into_os_string()
    }
}

impl TryFrom<(Vec<u8>, PathBuf)> for ProfileTools {
    type Error = Error;

    fn try_from(
        (library_output, profile_data_path): (Vec<u8>, PathBuf),
    ) -> Result<Self, Self::Error> {
        let library_text = String::from_utf8(library_output)?;
        let library_directory_text = library_text.trim();
        if library_directory_text.is_empty() {
            return Err(Error::LibraryDirectoryAbsence);
        }
        let library_directory = Path::new(library_directory_text);
        let tool_directory = library_directory.with_file_name(Self::TOOL_DIRECTORY_NAME);
        let profile_tools = Self {
            tool_directory,
            profile_data_path,
        };
        Ok(profile_tools)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

        pub(crate) fn profile_tools_factory() -> ProfileTools {
            let library_output = b"/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/lib\n".to_vec();
            let profile_data_path = PathBuf::from("/tmp/coverage-7/coverage.profdata");
            ProfileTools::try_from((library_output, profile_data_path)).unwrap()
        }

        pub(crate) fn tool_word_factory(tool_name: &str) -> OsString {
            let tool_path_text = format!(
                "/sysroot/lib/rustlib/x86_64-unknown-linux-gnu/bin/{tool_name}{EXE_SUFFIX}"
            );
            OsString::from(tool_path_text)
        }
    }

    #[test]
    fn the_merge_merges_every_profile_into_the_profile_data() {
        let profile_tools = fixture::profile_tools_factory();
        let first_profile_path = PathBuf::from("/tmp/coverage-7/11-22.profraw");
        let last_profile_path = PathBuf::from("/tmp/coverage-7/12-22.profraw");
        let profile_paths = [first_profile_path, last_profile_path];

        let invocation = profile_tools.compose_merge_invocation(&profile_paths);

        let program = fixture::tool_word_factory("llvm-profdata");
        let argument_texts = [
            "merge",
            "-sparse",
            "/tmp/coverage-7/11-22.profraw",
            "/tmp/coverage-7/12-22.profraw",
            "-o",
            "/tmp/coverage-7/coverage.profdata",
        ];
        let arguments = Vec::from_iter(argument_texts.map(OsString::from));
        let variables = Vec::new();
        let stream = Stream::Inheritance;
        let expectation = Invocation::from((program, arguments, variables, stream));
        assert_eq!(invocation, expectation);
    }

    #[test]
    fn the_export_names_the_first_executable_then_every_other_as_an_object_and_the_sources() {
        let profile_tools = fixture::profile_tools_factory();
        let first_executable = PathBuf::from("/target/debug/deps/harbor-0a1b");
        let last_executable = PathBuf::from("/target/debug/deps/quay-2c3d");
        let executables = vec![first_executable, last_executable];
        let manifest_path = PathBuf::from("/workspace/crates/harbor/Cargo.toml");
        let test_build = TestBuild::from((executables, manifest_path));

        let invocation = profile_tools.compose_export_invocation(&test_build);

        let program = fixture::tool_word_factory("llvm-cov");
        let argument_texts = [
            "export",
            "-format=text",
            "-instr-profile=/tmp/coverage-7/coverage.profdata",
            "/target/debug/deps/harbor-0a1b",
            "-object",
            "/target/debug/deps/quay-2c3d",
            "/workspace/crates/harbor/src",
        ];
        let arguments = Vec::from_iter(argument_texts.map(OsString::from));
        let variables = Vec::new();
        let stream = Stream::Capture;
        let expectation = Invocation::from((program, arguments, variables, stream));
        assert_eq!(invocation, expectation);
    }

    #[test]
    fn the_inspection_lists_the_section_headers_of_every_executable() {
        let profile_tools = fixture::profile_tools_factory();
        let first_executable = PathBuf::from("/target/debug/deps/harbor-0a1b");
        let last_executable = PathBuf::from("/target/debug/deps/quay-2c3d");
        let executables = vec![first_executable, last_executable];
        let manifest_path = PathBuf::from("/workspace/crates/harbor/Cargo.toml");
        let test_build = TestBuild::from((executables, manifest_path));

        let invocation = profile_tools.compose_inspection_invocation(&test_build);

        let program = fixture::tool_word_factory("llvm-objdump");
        let argument_texts = [
            "--section-headers",
            "/target/debug/deps/harbor-0a1b",
            "/target/debug/deps/quay-2c3d",
        ];
        let arguments = Vec::from_iter(argument_texts.map(OsString::from));
        let variables = Vec::new();
        let stream = Stream::Capture;
        let expectation = Invocation::from((program, arguments, variables, stream));
        assert_eq!(invocation, expectation);
    }

    #[test]
    fn an_empty_library_directory_is_an_error() {
        let library_output = b"\n".to_vec();
        let profile_data_path = PathBuf::from("/tmp/coverage-7/coverage.profdata");

        let profile_tools = ProfileTools::try_from((library_output, profile_data_path));

        let expectation = Err(Error::LibraryDirectoryAbsence);
        assert_eq!(profile_tools, expectation);
    }

    #[test]
    fn a_library_directory_that_is_no_text_is_an_error() {
        let library_output = vec![0xff, 0xfe];
        let profile_data_path = PathBuf::from("/tmp/coverage-7/coverage.profdata");

        let profile_tools = ProfileTools::try_from((library_output, profile_data_path));

        let expectation = Err(Error::LibraryDirectoryEncoding);
        assert_eq!(profile_tools, expectation);
    }
}
