mod profile_store;

pub use profile_store::ProfileStore;

use gates_coverage_core_invocation::Variable;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::ffi::OsString;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ProfileDirectory {
    path: PathBuf,
}

impl Layer for ProfileDirectory {
    type Layer = DomainLayer;
}

impl ValueObject for ProfileDirectory {}

impl ProfileDirectory {
    const PROFILE_VARIABLE_NAME: &'static str = "LLVM_PROFILE_FILE";

    const PROFILE_NAME_PATTERN: &'static str = "%p-%m.profraw";

    const PROFILE_DATA_NAME: &'static str = "coverage.profdata";

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn compose_profile_variable(&self) -> Variable {
        let name = OsString::from(Self::PROFILE_VARIABLE_NAME);
        let value = self.path.join(Self::PROFILE_NAME_PATTERN).into_os_string();
        Variable::from((name, value))
    }

    pub fn compose_profile_data_path(&self) -> PathBuf {
        self.path.join(Self::PROFILE_DATA_NAME)
    }
}

impl From<PathBuf> for ProfileDirectory {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_profile_directory_keeps_its_path() {
        let path = PathBuf::from("/tmp/coverage-7");

        let profile_directory = ProfileDirectory::from(path);

        let expectation = Path::new("/tmp/coverage-7");
        assert_eq!(profile_directory.path(), expectation);
    }

    #[test]
    fn the_profile_variable_names_a_profile_for_every_process_and_binary_in_the_directory() {
        let path = PathBuf::from("/tmp/coverage-7");
        let profile_directory = ProfileDirectory::from(path);

        let variable = profile_directory.compose_profile_variable();

        let name = OsString::from("LLVM_PROFILE_FILE");
        let value = OsString::from("/tmp/coverage-7/%p-%m.profraw");
        let expectation = Variable::from((name, value));
        assert_eq!(variable, expectation);
    }

    #[test]
    fn the_profile_data_lies_in_the_directory() {
        let path = PathBuf::from("/tmp/coverage-7");
        let profile_directory = ProfileDirectory::from(path);

        let profile_data_path = profile_directory.compose_profile_data_path();

        let expectation = PathBuf::from("/tmp/coverage-7/coverage.profdata");
        assert_eq!(profile_data_path, expectation);
    }
}
