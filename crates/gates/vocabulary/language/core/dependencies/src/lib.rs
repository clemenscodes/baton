mod dependency_list;
mod error;

pub mod decoder;
pub mod manifest;

pub use dependency_list::DependencyList;
pub use error::Error;

use crate::manifest::Manifest;
use crate::manifest::entries::Entries;
use gates_vocabulary_language_core_language::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Dependencies {
    package_identifier: Identifier,
    production_dependencies: DependencyList,
    development_dependencies: DependencyList,
}

impl Layer for Dependencies {
    type Layer = DomainLayer;
}

impl ValueObject for Dependencies {}

impl Dependencies {
    pub fn package_identifier(&self) -> &Identifier {
        &self.package_identifier
    }

    pub fn production_dependencies(&self) -> &DependencyList {
        &self.production_dependencies
    }

    pub fn development_dependencies(&self) -> &DependencyList {
        &self.development_dependencies
    }
}

impl From<&[Manifest]> for Dependencies {
    fn from(manifests: &[Manifest]) -> Self {
        match manifests {
            [] => Self::default(),
            [manifest] => {
                let workspace_entries = Entries::default();
                Self::from((manifest, &workspace_entries))
            }
            [manifest, root_manifest, ..] => {
                let workspace_entries = root_manifest.workspace_entries();
                Self::from((manifest, workspace_entries))
            }
        }
    }
}

impl From<(&Manifest, &Entries)> for Dependencies {
    fn from((manifest, workspace_entries): (&Manifest, &Entries)) -> Self {
        let package_identifier = manifest.package_identifier().clone();
        let dependency_entries = manifest.dependency_entries();
        let development_dependency_entries = manifest.development_dependency_entries();
        let production_dependencies = DependencyList::from((dependency_entries, workspace_entries));
        let development_dependencies = DependencyList::from((
            development_dependency_entries,
            workspace_entries,
        ));
        Self {
            package_identifier,
            production_dependencies,
            development_dependencies,
        }
    }
}

impl From<(Identifier, DependencyList, DependencyList)> for Dependencies {
    fn from(
        (package_identifier, production_dependencies, development_dependencies): (
            Identifier,
            DependencyList,
            DependencyList,
        ),
    ) -> Self {
        Self {
            package_identifier,
            production_dependencies,
            development_dependencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::entries::tests::fixture as entry_fixture;
    use gates_vocabulary_language_core_foreign_name::ForeignName;
    use std::path::{Path, PathBuf};

    #[test]
    fn no_manifest_names_no_crate_and_no_path() {
        let manifests: &[Manifest] = &[];

        let dependencies = Dependencies::from(manifests);

        let expectation = Dependencies::default();
        assert_eq!(dependencies, expectation);
    }

    #[test]
    fn a_manifest_names_the_crates_its_dependencies_and_its_development_dependencies_list() {
        let dependency_name_text = "syn";
        let development_dependency_name_text = "tokio";
        let dependency_entry = entry_fixture::foreign_name_entry_factory(dependency_name_text);
        let development_dependency_entry = entry_fixture::foreign_name_entry_factory(
            development_dependency_name_text,
        );
        let dependency_list = vec![dependency_entry];
        let development_dependency_list = vec![development_dependency_entry];
        let package_identifier = Identifier::default();
        let dependency_entries = Entries::try_from(dependency_list).unwrap();
        let development_dependency_entries = Entries::try_from(development_dependency_list)
            .unwrap();
        let workspace_entries = Entries::default();
        let manifest = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        let manifest_list = [manifest];
        let manifests = manifest_list.as_slice();

        let dependencies = Dependencies::from(manifests);

        let production_dependencies = dependencies.production_dependencies();
        let development_dependencies = dependencies.development_dependencies();
        let production_foreign_names = production_dependencies.foreign_names();
        let development_dependency_names = development_dependencies.foreign_names();
        let dependency_name = ForeignName::from(dependency_name_text);
        let development_dependency_name = ForeignName::from(development_dependency_name_text);
        let production_expectation = [dependency_name];
        let development_dependency_expectation = [development_dependency_name];
        assert_eq!(production_foreign_names, production_expectation);
        assert_eq!(
            development_dependency_names,
            development_dependency_expectation
        );
    }

    #[test]
    fn a_dependency_of_the_root_manifest_leads_to_the_path_the_root_manifest_names() {
        let foreign_name_text = "ship";
        let crate_path_text = "crates/ship";
        let foreign_name_entry = entry_fixture::foreign_name_entry_factory(foreign_name_text);
        let workspace_entry = entry_fixture::path_entry_factory(crate_path_text);
        let dependency_list = vec![foreign_name_entry];
        let workspace_list = vec![workspace_entry];
        let package_identifier = Identifier::default();
        let dependency_entries = Entries::try_from(dependency_list).unwrap();
        let development_dependency_entries = Entries::default();
        let manifest_workspace_entries = Entries::default();
        let manifest = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            manifest_workspace_entries,
        ));
        let root_package_identifier = Identifier::default();
        let root_dependency_entries = Entries::default();
        let root_development_dependency_entries = Entries::default();
        let workspace_entries = Entries::try_from(workspace_list).unwrap();
        let root_manifest = Manifest::from((
            root_package_identifier,
            root_dependency_entries,
            root_development_dependency_entries,
            workspace_entries,
        ));
        let manifest_list = [manifest, root_manifest];
        let manifests = manifest_list.as_slice();

        let dependencies = Dependencies::from(manifests);

        let production_dependencies = dependencies.production_dependencies();
        let paths = production_dependencies.paths();
        let crate_path = Path::new(crate_path_text);
        let expectation = [crate_path];
        assert_eq!(paths, expectation);
    }

    #[test]
    fn the_package_identifier_of_the_nearest_manifest_is_the_package_identifier_of_the_crate() {
        let package_identifier_text = "stack_game_context_combat";
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let dependency_entries = Entries::default();
        let development_dependency_entries = Entries::default();
        let workspace_entries = Entries::default();
        let manifest = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        let manifest_list = [manifest];
        let manifests = manifest_list.as_slice();

        let dependencies = Dependencies::from(manifests);

        let dependency_package_identifier = dependencies.package_identifier();
        let identifier_expectation = Identifier::try_from(package_identifier_text).unwrap();
        let expectation = &identifier_expectation;
        assert_eq!(dependency_package_identifier, expectation);
    }

    #[test]
    fn dependencies_of_an_identifier_and_two_dependency_lists_keep_all_three() {
        let package_identifier_text = "stack_game_context_combat";
        let production_path_text = "crates/ship";
        let development_dependency_path_text = "crates/harbor";
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let production_path = PathBuf::from(production_path_text);
        let development_dependency_path = PathBuf::from(development_dependency_path_text);
        let production_dependencies = DependencyList::from_iter([production_path]);
        let development_dependencies = DependencyList::from_iter([development_dependency_path]);

        let dependencies = Dependencies::from((
            package_identifier,
            production_dependencies,
            development_dependencies,
        ));

        let dependency_package_identifier = dependencies.package_identifier();
        let production_paths = dependencies.production_dependencies().paths();
        let development_dependency_paths = dependencies.development_dependencies().paths();
        let identifier_expectation = Identifier::try_from(package_identifier_text).unwrap();
        let production_expectation = [Path::new(production_path_text)];
        let development_dependency_expectation = [Path::new(development_dependency_path_text)];
        assert_eq!(dependency_package_identifier, &identifier_expectation);
        assert_eq!(production_paths, production_expectation);
        assert_eq!(
            development_dependency_paths,
            development_dependency_expectation
        );
    }
}
