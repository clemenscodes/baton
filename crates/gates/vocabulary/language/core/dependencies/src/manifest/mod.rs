mod text;

pub mod entries;

pub use text::ManifestText;

use crate::manifest::entries::Entries;
use gates_vocabulary_language_core_language::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Manifest {
    package_identifier: Identifier,
    dependency_entries: Entries,
    development_dependency_entries: Entries,
    workspace_entries: Entries,
}

impl Layer for Manifest {
    type Layer = DomainLayer;
}

impl ValueObject for Manifest {}

impl Manifest {
    pub(super) fn package_identifier(&self) -> &Identifier {
        &self.package_identifier
    }

    pub(super) fn dependency_entries(&self) -> &Entries {
        &self.dependency_entries
    }

    pub(super) fn development_dependency_entries(&self) -> &Entries {
        &self.development_dependency_entries
    }

    pub(super) fn workspace_entries(&self) -> &Entries {
        &self.workspace_entries
    }
}

impl From<(Identifier, Entries, Entries, Entries)> for Manifest {
    fn from(
        (package_identifier, dependency_entries, development_dependency_entries, workspace_entries): (
            Identifier,
            Entries,
            Entries,
            Entries,
        ),
    ) -> Self {
        Self {
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        }
    }
}

impl From<(&Manifest, &Path)> for Manifest {
    fn from((manifest, directory): (&Manifest, &Path)) -> Self {
        let Manifest {
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        } = manifest;
        let directory_package_identifier = package_identifier.clone();
        let directory_dependency_entries = Entries::from((dependency_entries, directory));
        let directory_development_dependency_entries = Entries::from((
            development_dependency_entries,
            directory,
        ));
        let directory_workspace_entries = Entries::from((workspace_entries, directory));
        Self {
            package_identifier: directory_package_identifier,
            dependency_entries: directory_dependency_entries,
            development_dependency_entries: directory_development_dependency_entries,
            workspace_entries: directory_workspace_entries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::entries::tests::fixture as entry_fixture;

    #[test]
    fn a_manifest_of_a_directory_leads_its_entries_into_that_directory() {
        let crate_path_text = "crates/ship";
        let directory_text = "/workspace";
        let package_identifier_text = "ship";
        let dependency_entry = entry_fixture::path_entry_factory(crate_path_text);
        let development_dependency_entry = entry_fixture::path_entry_factory(crate_path_text);
        let workspace_entry = entry_fixture::path_entry_factory(crate_path_text);
        let dependency_list = vec![dependency_entry];
        let development_dependency_list = vec![development_dependency_entry];
        let workspace_list = vec![workspace_entry];
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let dependency_entries = Entries::try_from(dependency_list).unwrap();
        let development_dependency_entries = Entries::try_from(development_dependency_list)
            .unwrap();
        let workspace_entries = Entries::try_from(workspace_list).unwrap();
        let manifest = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        let directory = Path::new(directory_text);

        let directory_manifest = Manifest::from((&manifest, directory));

        let directory_path_text = "/workspace/crates/ship";
        let directory_dependency_entry = entry_fixture::path_entry_factory(directory_path_text);
        let directory_development_dependency_entry = entry_fixture::path_entry_factory(
            directory_path_text,
        );
        let directory_workspace_entry = entry_fixture::path_entry_factory(directory_path_text);
        let directory_dependency_list = vec![directory_dependency_entry];
        let directory_development_dependency_list = vec![directory_development_dependency_entry];
        let directory_workspace_list = vec![directory_workspace_entry];
        let expectation_package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let expectation_dependency_entries = Entries::try_from(directory_dependency_list).unwrap();
        let expectation_development_dependency_entries = Entries::try_from(
            directory_development_dependency_list,
        )
        .unwrap();
        let expectation_workspace_entries = Entries::try_from(directory_workspace_list).unwrap();
        let expectation = Manifest::from((
            expectation_package_identifier,
            expectation_dependency_entries,
            expectation_development_dependency_entries,
            expectation_workspace_entries,
        ));
        assert_eq!(directory_manifest, expectation);
    }
}
