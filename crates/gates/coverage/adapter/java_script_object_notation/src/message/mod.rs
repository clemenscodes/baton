mod artifact_profile;

use artifact_profile::ArtifactProfile;

use super::Error;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
pub(super) struct Message {
    reason: String,
    #[serde(default)]
    executable: Option<PathBuf>,
    #[serde(default)]
    manifest_path: Option<PathBuf>,
    #[serde(default, rename = "profile")]
    artifact_profile: Option<ArtifactProfile>,
}

impl Layer for Message {
    type Layer = InfrastructureLayer;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct TestExecutable {
    executable: PathBuf,
    manifest_path: PathBuf,
}

impl Layer for TestExecutable {
    type Layer = InfrastructureLayer;
}

impl TestExecutable {
    const ARTIFACT_REASON: &'static str = "compiler-artifact";

    pub(super) fn executable(&self) -> &PathBuf {
        &self.executable
    }

    pub(super) fn manifest_path(&self) -> &PathBuf {
        &self.manifest_path
    }
}

impl TryFrom<Message> for TestExecutable {
    type Error = Error;

    fn try_from(message: Message) -> Result<Self, Self::Error> {
        let Message {
            reason,
            executable,
            manifest_path,
            artifact_profile,
        } = message;
        let test_flag = artifact_profile
            .as_ref()
            .is_some_and(ArtifactProfile::test_flag);
        if reason != Self::ARTIFACT_REASON || !test_flag {
            return Err(Error::TestExecutableAbsence);
        }
        let executable = executable.ok_or(Error::TestExecutableAbsence)?;
        let manifest_path = manifest_path.ok_or(Error::TestExecutableAbsence)?;
        let test_executable = Self {
            executable,
            manifest_path,
        };
        Ok(test_executable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_artifact_of_a_test_with_an_executable_is_a_test_executable() {
        let text = r#"{"reason":"compiler-artifact","manifest_path":"/workspace/crates/harbor/Cargo.toml","profile":{"test":true},"executable":"/target/debug/deps/harbor-0a1b"}"#;
        let message: Message = serde_json::from_str(text).unwrap();

        let test_executable = TestExecutable::try_from(message).unwrap();

        let executable_expectation = PathBuf::from("/target/debug/deps/harbor-0a1b");
        let manifest_path_expectation = PathBuf::from("/workspace/crates/harbor/Cargo.toml");
        assert_eq!(test_executable.executable(), &executable_expectation);
        assert_eq!(test_executable.manifest_path(), &manifest_path_expectation);
    }

    #[test]
    fn an_artifact_that_is_no_test_is_no_test_executable() {
        let text = r#"{"reason":"compiler-artifact","manifest_path":"/workspace/crates/harbor/Cargo.toml","profile":{"test":false},"executable":null}"#;
        let message: Message = serde_json::from_str(text).unwrap();

        let test_executable = TestExecutable::try_from(message);

        assert!(matches!(test_executable, Err(Error::TestExecutableAbsence)));
    }

    #[test]
    fn the_end_of_a_build_is_no_test_executable() {
        let text = r#"{"reason":"build-finished","success":true}"#;
        let message: Message = serde_json::from_str(text).unwrap();

        let test_executable = TestExecutable::try_from(message);

        assert!(matches!(test_executable, Err(Error::TestExecutableAbsence)));
    }

    #[test]
    fn an_artifact_of_a_test_without_an_executable_is_no_test_executable() {
        let text = r#"{"reason":"compiler-artifact","manifest_path":"/workspace/crates/harbor/Cargo.toml","profile":{"test":true},"executable":null}"#;
        let message: Message = serde_json::from_str(text).unwrap();

        let test_executable = TestExecutable::try_from(message);

        assert!(matches!(test_executable, Err(Error::TestExecutableAbsence)));
    }

    #[test]
    fn an_artifact_of_a_test_without_a_manifest_is_no_test_executable() {
        let text = r#"{"reason":"compiler-artifact","profile":{"test":true},"executable":"/target/debug/deps/harbor-0a1b"}"#;
        let message: Message = serde_json::from_str(text).unwrap();

        let test_executable = TestExecutable::try_from(message);

        assert!(matches!(test_executable, Err(Error::TestExecutableAbsence)));
    }
}
