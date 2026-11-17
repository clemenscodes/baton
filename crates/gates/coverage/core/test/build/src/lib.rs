use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TestBuild {
    executables: Vec<PathBuf>,
    manifest_path: PathBuf,
}

impl Layer for TestBuild {
    type Layer = DomainLayer;
}

impl ValueObject for TestBuild {}

impl TestBuild {
    const SOURCE_DIRECTORY_NAME: &'static str = "src";

    pub fn executables(&self) -> &[PathBuf] {
        &self.executables
    }

    pub fn compose_source_directory(&self) -> PathBuf {
        let manifest_path = &self.manifest_path;
        let package_directory = manifest_path.with_file_name("");
        package_directory.join(Self::SOURCE_DIRECTORY_NAME)
    }
}

impl From<(Vec<PathBuf>, PathBuf)> for TestBuild {
    fn from((executables, manifest_path): (Vec<PathBuf>, PathBuf)) -> Self {
        Self {
            executables,
            manifest_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test_build_keeps_its_executables() {
        let executable = PathBuf::from("/target/debug/deps/harbor-0a1b");
        let executables = vec![executable];
        let manifest_path = PathBuf::from("/workspace/crates/harbor/Cargo.toml");

        let test_build = TestBuild::from((executables, manifest_path));

        let expectation = PathBuf::from("/target/debug/deps/harbor-0a1b");
        assert_eq!(test_build.executables(), [expectation]);
    }

    #[test]
    fn the_source_directory_lies_beside_the_manifest_of_the_package() {
        let executable = PathBuf::from("/target/debug/deps/harbor-0a1b");
        let executables = vec![executable];
        let manifest_path = PathBuf::from("/workspace/crates/harbor/Cargo.toml");
        let test_build = TestBuild::from((executables, manifest_path));

        let source_directory = test_build.compose_source_directory();

        let expectation = PathBuf::from("/workspace/crates/harbor/src");
        assert_eq!(source_directory, expectation);
    }
}
