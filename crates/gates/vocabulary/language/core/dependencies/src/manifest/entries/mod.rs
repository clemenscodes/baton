mod entry;

pub use entry::Entry;

use crate::Error;
use gates_vocabulary_language_core_foreign_name::ForeignName;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeMap;
use std::collections::btree_map::Values;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Entries {
    entries: BTreeMap<ForeignName, Entry>,
}

impl Layer for Entries {
    type Layer = DomainLayer;
}

impl ValueObject for Entries {}

impl Entries {
    pub(crate) fn maybe_entry(&self, foreign_name: &ForeignName) -> Option<&Entry> {
        match self {
            Self { entries } => entries.get(foreign_name),
        }
    }
}

impl TryFrom<Vec<Entry>> for Entries {
    type Error = Error;

    fn try_from(entry_list: Vec<Entry>) -> Result<Self, Self::Error> {
        let mut entries = BTreeMap::new();
        for entry in entry_list {
            let foreign_name = entry.foreign_name().clone();
            match entries.contains_key(&foreign_name) {
                true => {
                    let error = Error::Entry { foreign_name };
                    return Err(error);
                }
                false => {
                    entries.insert(foreign_name, entry);
                }
            }
        }
        let dependency_entries = Self { entries };
        Ok(dependency_entries)
    }
}

impl From<(&Entries, &Path)> for Entries {
    fn from((entries, directory): (&Entries, &Path)) -> Self {
        let mut directory_entries = BTreeMap::new();
        for entry in entries {
            let foreign_name = entry.foreign_name().clone();
            let directory_entry = Entry::from((entry, directory));
            directory_entries.insert(foreign_name, directory_entry);
        }
        Self {
            entries: directory_entries,
        }
    }
}

impl<'entries> IntoIterator for &'entries Entries {
    type Item = &'entries Entry;
    type IntoIter = Values<'entries, ForeignName, Entry>;

    fn into_iter(self) -> Self::IntoIter {
        let Entries { entries } = self;
        entries.values()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) mod fixture {
        use super::*;

        pub(crate) fn foreign_name_entry_factory(foreign_name_text: &str) -> Entry {
            let foreign_name = ForeignName::from(foreign_name_text);
            let path = None;
            Entry::from((foreign_name, path))
        }

        pub(crate) fn path_entry_factory(crate_path_text: &str) -> Entry {
            let crate_path = Path::new(crate_path_text);
            let directory_name = crate_path.file_name().unwrap();
            let foreign_name_text = directory_name.to_str().unwrap();
            let foreign_name = ForeignName::from(foreign_name_text);
            let entry_path = crate_path.to_path_buf();
            let path = Some(entry_path);
            Entry::from((foreign_name, path))
        }
    }

    #[test]
    fn the_entries_yield_the_entry_of_a_foreign_name() {
        let parser_name_text = "syn";
        let decoder_name_text = "toml";
        let parser_entry = fixture::foreign_name_entry_factory(parser_name_text);
        let decoder_entry = fixture::foreign_name_entry_factory(decoder_name_text);
        let entry_list = vec![parser_entry, decoder_entry];
        let entries = Entries::try_from(entry_list).unwrap();
        let foreign_name = ForeignName::from(decoder_name_text);

        let entry = entries.maybe_entry(&foreign_name);

        let decoder_expectation = fixture::foreign_name_entry_factory(decoder_name_text);
        let expectation = Some(&decoder_expectation);
        assert_eq!(entry, expectation);
    }

    #[test]
    fn a_second_entry_of_one_foreign_name_is_an_error() {
        let foreign_name_text = "language";
        let foreign_name_entry = fixture::foreign_name_entry_factory(foreign_name_text);
        let crate_path_text = "crates/language";
        let path_entry = fixture::path_entry_factory(crate_path_text);
        let entry_list = vec![foreign_name_entry, path_entry];

        let entries = Entries::try_from(entry_list);

        let foreign_name = ForeignName::from(foreign_name_text);
        let error = Error::Entry { foreign_name };
        let expectation = Err(error);
        assert_eq!(entries, expectation);
    }

    #[test]
    fn the_entries_of_a_directory_lead_to_their_paths_inside_that_directory() {
        let crate_path_text = "crates/language";
        let directory_text = "/workspace";
        let path_entry = fixture::path_entry_factory(crate_path_text);
        let entry_list = vec![path_entry];
        let entries = Entries::try_from(entry_list).unwrap();
        let directory = Path::new(directory_text);

        let directory_entries = Entries::from((&entries, directory));

        let directory_path_text = "/workspace/crates/language";
        let directory_path_entry = fixture::path_entry_factory(directory_path_text);
        let directory_entry_list = vec![directory_path_entry];
        let expectation = Entries::try_from(directory_entry_list).unwrap();
        assert_eq!(directory_entries, expectation);
    }
}
