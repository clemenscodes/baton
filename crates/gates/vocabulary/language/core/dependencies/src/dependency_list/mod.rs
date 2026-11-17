use crate::manifest::entries::{Entries, Entry};
use gates_vocabulary_language_core_foreign_name::ForeignName;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::ops::BitOr;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct DependencyList {
    foreign_names: Vec<ForeignName>,
    paths: Vec<PathBuf>,
}

impl Layer for DependencyList {
    type Layer = DomainLayer;
}

impl ValueObject for DependencyList {}

impl DependencyList {
    pub fn foreign_names(&self) -> &[ForeignName] {
        match self {
            Self { foreign_names, .. } => foreign_names,
        }
    }

    pub fn paths(&self) -> &[PathBuf] {
        match self {
            Self { paths, .. } => paths,
        }
    }
}

impl From<(&Entries, &Entries)> for DependencyList {
    fn from((dependency_entries, workspace_entries): (&Entries, &Entries)) -> Self {
        let mut foreign_names = Vec::new();
        let mut paths = Vec::new();
        for entry in dependency_entries {
            let foreign_name = entry.foreign_name();
            let workspace_entry = workspace_entries.maybe_entry(foreign_name);
            let workspace_path = workspace_entry.and_then(Entry::maybe_path);
            let entry_path = entry.maybe_path();
            if let Some(path) = entry_path.or(workspace_path) {
                let dependency_path = path.to_path_buf();
                paths.push(dependency_path);
            }
            let dependency_name = foreign_name.clone();
            foreign_names.push(dependency_name);
        }
        Self {
            foreign_names,
            paths,
        }
    }
}

impl FromIterator<PathBuf> for DependencyList {
    fn from_iter<PathsType: IntoIterator<Item = PathBuf>>(path_sequence: PathsType) -> Self {
        let foreign_names = Vec::new();
        let paths = path_sequence.into_iter().collect();
        Self {
            foreign_names,
            paths,
        }
    }
}

impl BitOr for &DependencyList {
    type Output = DependencyList;

    fn bitor(self, dependency_list: Self) -> Self::Output {
        let DependencyList {
            foreign_names: augend_foreign_names,
            paths: augend_paths,
        } = self;
        let DependencyList {
            foreign_names: addend_foreign_names,
            paths: addend_paths,
        } = dependency_list;
        let foreign_names = augend_foreign_names
            .iter()
            .chain(addend_foreign_names)
            .cloned()
            .collect();
        let paths = augend_paths.iter().chain(addend_paths).cloned().collect();
        DependencyList {
            foreign_names,
            paths,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::entries::tests::fixture as entry_fixture;
    use std::path::Path;

    #[test]
    fn a_dependency_that_names_its_own_path_leads_to_that_path() {
        let crate_path_text = "crates/ship";
        let path_entry = entry_fixture::path_entry_factory(crate_path_text);
        let entry_list = vec![path_entry];
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let workspace_entries = Entries::default();

        let dependency_list = DependencyList::from((&dependency_entries, &workspace_entries));

        let paths = dependency_list.paths();
        let crate_path = Path::new(crate_path_text);
        let expectation = [crate_path];
        assert_eq!(paths, expectation);
    }

    #[test]
    fn a_dependency_the_workspace_names_no_path_for_leads_nowhere() {
        let foreign_name_text = "syn";
        let foreign_name_entry = entry_fixture::foreign_name_entry_factory(foreign_name_text);
        let workspace_entry = entry_fixture::foreign_name_entry_factory(foreign_name_text);
        let entry_list = vec![foreign_name_entry];
        let workspace_list = vec![workspace_entry];
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let workspace_entries = Entries::try_from(workspace_list).unwrap();

        let dependency_list = DependencyList::from((&dependency_entries, &workspace_entries));

        let paths = dependency_list.paths();
        let expectation: &[PathBuf] = &[];
        assert_eq!(paths, expectation);
    }

    #[test]
    fn a_dependency_of_another_name_than_the_workspace_names_leads_nowhere() {
        let foreign_name_text = "ship";
        let crate_path_text = "crates/lighthouse";
        let foreign_name_entry = entry_fixture::foreign_name_entry_factory(foreign_name_text);
        let workspace_entry = entry_fixture::path_entry_factory(crate_path_text);
        let entry_list = vec![foreign_name_entry];
        let workspace_list = vec![workspace_entry];
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let workspace_entries = Entries::try_from(workspace_list).unwrap();

        let dependency_list = DependencyList::from((&dependency_entries, &workspace_entries));

        let paths = dependency_list.paths();
        let expectation: &[PathBuf] = &[];
        assert_eq!(paths, expectation);
    }

    #[test]
    fn a_dependency_list_of_paths_names_no_foreign_name() {
        let crate_path_text = "crates/ship";
        let crate_path = PathBuf::from(crate_path_text);
        let path_list = [crate_path];

        let dependency_list: DependencyList = path_list.into_iter().collect();

        let foreign_names = dependency_list.foreign_names();
        let expectation: &[ForeignName] = &[];
        assert_eq!(foreign_names, expectation);
    }

    #[test]
    fn two_joined_dependency_lists_hold_the_foreign_names_and_the_paths_of_both() {
        let foreign_name_text = "syn";
        let crate_path_text = "crates/ship";
        let foreign_name_entry = entry_fixture::foreign_name_entry_factory(foreign_name_text);
        let entry_list = vec![foreign_name_entry];
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let workspace_entries = Entries::default();
        let augend = DependencyList::from((&dependency_entries, &workspace_entries));
        let crate_path = PathBuf::from(crate_path_text);
        let path_list = [crate_path];
        let addend: DependencyList = path_list.into_iter().collect();

        let dependency_list = &augend | &addend;

        let foreign_names = dependency_list.foreign_names();
        let paths = dependency_list.paths();
        let foreign_name = ForeignName::from(foreign_name_text);
        let crate_path_expectation = Path::new(crate_path_text);
        let foreign_name_expectation = [foreign_name];
        let path_expectation = [crate_path_expectation];
        assert_eq!(foreign_names, foreign_name_expectation);
        assert_eq!(paths, path_expectation);
    }
}
