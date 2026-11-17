mod book_entry;

pub use book_entry::BookEntry;

use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Debug, Default)]
pub struct Edition {
    book_entries: Vec<BookEntry>,
}

impl Layer for Edition {
    type Layer = ApplicationLayer;
}

impl FromIterator<BookEntry> for Edition {
    fn from_iter<BookEntriesType: IntoIterator<Item = BookEntry>>(
        book_entry_sequence: BookEntriesType,
    ) -> Self {
        let book_entries = book_entry_sequence.into_iter().collect();
        Self { book_entries }
    }
}

impl IntoIterator for Edition {
    type Item = BookEntry;
    type IntoIter = IntoIter<BookEntry>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { book_entries } = self;
        book_entries.into_iter()
    }
}

impl<'edition> IntoIterator for &'edition Edition {
    type Item = &'edition BookEntry;
    type IntoIter = Iter<'edition, BookEntry>;

    fn into_iter(self) -> Self::IntoIter {
        let Edition { book_entries } = self;
        book_entries.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn an_edition_yields_every_entry_it_holds_by_value_and_by_reference() {
        let path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let path = PathBuf::from(path_text);
        let omission = BookEntry::Omission { path };
        let book_entry_list = [omission];
        let edition: Edition = book_entry_list.into_iter().collect();

        let borrowed_texts: Vec<String> = (&edition).into_iter().map(ToString::to_string).collect();
        let owned_texts: Vec<String> = edition.into_iter().map(|entry| entry.to_string()).collect();

        let expectation = [
            "[UBIQUITOUS] skipped `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: its directory does not exist.",
        ];
        assert_eq!(borrowed_texts, expectation);
        assert_eq!(owned_texts, expectation);
    }
}
