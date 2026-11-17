pub mod edition;

mod book_edition;

pub use book_edition::BookEdition;

use crate::Error;
use crate::press::edition::{BookEntry, Edition};
use crate::vocabulary::Vocabulary;
use gates_vocabulary_language_core_books::writer::{DirectoryPresence, Writer};
use gates_vocabulary_language_core_books::{Book, Books};
use gates_vocabulary_language_core_languages::Languages;
use stack_domain_driven_design_core::application_service::ApplicationService;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Press<WriterType> {
    languages: Languages,
    writer: WriterType,
}

impl<WriterType> Layer for Press<WriterType> {
    type Layer = ApplicationLayer;
}

impl<WriterType> ApplicationService for Press<WriterType> {}

impl<WriterType: Writer> SynchronousCommandHandler<BookEdition> for Press<WriterType> {
    type Outcome = Result<Edition, Error>;

    fn dispatch_command(&self, book_edition: BookEdition) -> Self::Outcome {
        let directories = book_edition.directories();
        let books = self.try_assemble_books(directories)?;
        let book_entry = |book: &Book| self.compose_book_entry(book, directories);
        let edition = books.into_iter().map(book_entry).collect();
        Ok(edition)
    }
}

impl<WriterType: Writer> Press<WriterType> {
    fn compose_book_entry(&self, book: &Book, directories: &[PathBuf]) -> BookEntry {
        match directories {
            [] => self.compose_present_book_entry(book),
            [..] => self.compose_publication_entry(book),
        }
    }

    fn compose_present_book_entry(&self, book: &Book) -> BookEntry {
        let presence = self.try_detect_directory(book);
        match presence {
            Ok(DirectoryPresence::Presence) => self.compose_publication_entry(book),
            Ok(DirectoryPresence::Absence) => {
                let book_path = book.path();
                let path = book_path.to_path_buf();
                BookEntry::Omission { path }
            }
            Err(error) => BookEntry::Error { error },
        }
    }

    fn compose_publication_entry(&self, book: &Book) -> BookEntry {
        match self.try_write_book(book) {
            Ok(()) => {
                let book_path = book.path();
                let path = book_path.to_path_buf();
                let term_count = book.term_count();
                BookEntry::Publication { path, term_count }
            }
            Err(error) => BookEntry::Error { error },
        }
    }

    fn try_assemble_books(&self, directories: &[PathBuf]) -> Result<Books, Error> {
        let Self { languages, .. } = self;
        let contexts = languages.contexts();
        let mut book_list = Vec::new();
        match directories {
            [] => {
                for context in contexts {
                    let book = Book::from(context);
                    book_list.push(book);
                }
            }
            [..] => {
                for directory in directories {
                    let context = contexts.try_select_context(directory)?;
                    let book = Book::from(context);
                    book_list.push(book);
                }
            }
        }
        let books = book_list.into_iter().collect();
        Ok(books)
    }

    fn try_detect_directory(&self, book: &Book) -> Result<DirectoryPresence, Error> {
        let Self { writer, .. } = self;
        let directory = book.directory();
        let directory_error = |error: io::Error| {
            let path = directory.to_path_buf();
            Error::Directory { path, error }
        };
        writer
            .try_detect_directory(directory)
            .map_err(directory_error)
    }

    fn try_write_book(&self, book: &Book) -> Result<(), Error> {
        let Self { writer, .. } = self;
        let path = book.path();
        let book_error = |error: io::Error| {
            let book_path = path.to_path_buf();
            Error::Book {
                path: book_path,
                error,
            }
        };
        let text = book.to_string();
        let text_bytes = text.as_bytes();
        let mut file = writer.try_create_file(path).map_err(book_error)?;
        file.write_all(text_bytes).map_err(book_error)?;
        file.flush().map_err(book_error)?;
        writer.try_replace_file(path).map_err(book_error)
    }
}

impl<VocabularyType: Vocabulary, WriterType> TryFrom<(VocabularyType, WriterType)>
    for Press<WriterType>
{
    type Error = Error;

    fn try_from((vocabulary, writer): (VocabularyType, WriterType)) -> Result<Self, Self::Error> {
        let context_list = vocabulary.try_list_contexts()?;
        let foreign_language_list = vocabulary.try_list_foreign_languages()?;
        let press = |languages: Languages| Self { languages, writer };
        let languages = context_list.try_compose_languages(foreign_language_list);
        languages.map(press)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::gate::tests::fixture::VocabularyErrorAdapter;
    use gates_vocabulary_language_core_language::Language;
    use gates_vocabulary_language_core_language::TermText;
    use gates_vocabulary_language_core_languages::contexts::Context;
    use stack_domain_driven_design_core::layer::InfrastructureLayer;
    use stack_domain_driven_design_core::port::{DrivenPort, Port};
    use std::io::{Cursor, sink};
    use std::path::Path;

    pub(crate) mod fixture {
        use super::*;
        use crate::gate::tests::fixture::VocabularyAdapter;
        use stack_domain_driven_design_core::adapter::Adapter;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
        pub(crate) enum WriterAdapter {
            Presence,
            Absence,
            CreateFileError,
            WriteError,
            ReplaceError,
            DetectDirectoryError,
            FlushError,
        }

        impl Layer for WriterAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for WriterAdapter {}

        impl Port for WriterAdapter {}

        impl DrivenPort for WriterAdapter {}

        impl Writer for WriterAdapter {
            fn try_create_file(&self, _: &Path) -> io::Result<Box<dyn Write + Send>> {
                match self {
                    Self::CreateFileError => {
                        let error_text = "no file";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                    Self::FlushError => {
                        let flush_error_writer = FlushErrorWriter;
                        let writer: Box<dyn Write + Send> = Box::new(flush_error_writer);
                        Ok(writer)
                    }
                    Self::WriteError => {
                        let writer_bytes: [u8; 0] = [];
                        let cursor = Cursor::new(writer_bytes);
                        let writer: Box<dyn Write + Send> = Box::new(cursor);
                        Ok(writer)
                    }
                    Self::Presence
                    | Self::Absence
                    | Self::ReplaceError
                    | Self::DetectDirectoryError => {
                        let sink_writer = sink();
                        let writer: Box<dyn Write + Send> = Box::new(sink_writer);
                        Ok(writer)
                    }
                }
            }

            fn try_replace_file(&self, _: &Path) -> io::Result<()> {
                match self {
                    Self::ReplaceError => {
                        let error_text = "no room";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                    Self::Presence
                    | Self::Absence
                    | Self::CreateFileError
                    | Self::WriteError
                    | Self::DetectDirectoryError
                    | Self::FlushError => Ok(()),
                }
            }

            fn try_detect_directory(&self, _: &Path) -> io::Result<DirectoryPresence> {
                match self {
                    Self::Absence => {
                        let absence = DirectoryPresence::Absence;
                        Ok(absence)
                    }
                    Self::DetectDirectoryError => {
                        let error_text = "no sight";
                        let error = io::Error::other(error_text);
                        Err(error)
                    }
                    Self::Presence
                    | Self::CreateFileError
                    | Self::WriteError
                    | Self::ReplaceError
                    | Self::FlushError => {
                        let presence = DirectoryPresence::Presence;
                        Ok(presence)
                    }
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct FlushErrorWriter;

        impl Layer for FlushErrorWriter {
            type Layer = InfrastructureLayer;
        }

        impl Write for FlushErrorWriter {
            fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
                let byte_count = bytes.len();
                Ok(byte_count)
            }

            fn flush(&mut self) -> io::Result<()> {
                let error_text = "stuck";
                let error = io::Error::other(error_text);
                Err(error)
            }
        }

        pub(crate) fn press_factory(writer_adapter: WriterAdapter) -> Press<WriterAdapter> {
            const TERM_TEXTS: &[TermText] = &[{
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing.";
                TermText::new(spelling, definition)
            }];
            let term_texts = TERM_TEXTS;
            let language = Language::try_from(term_texts).unwrap();
            let directory_name = "harbor";
            let context = Context::from((directory_name, language));
            let contexts = vec![context];
            let foreign_languages = Vec::new();
            let vocabulary_adapter = VocabularyAdapter {
                contexts,
                foreign_languages,
            };
            Press::try_from((vocabulary_adapter, writer_adapter)).unwrap()
        }

        pub(super) fn empty_press_factory() -> Press<WriterAdapter> {
            let language = Language::default();
            let directory_name = "ship";
            let context = Context::from((directory_name, language));
            let contexts = vec![context];
            let foreign_languages = Vec::new();
            let writer_adapter = WriterAdapter::Presence;
            let vocabulary_adapter = VocabularyAdapter {
                contexts,
                foreign_languages,
            };
            Press::try_from((vocabulary_adapter, writer_adapter)).unwrap()
        }

        pub(super) fn book_factory() -> Book {
            const TERM_TEXTS: &[TermText] = &[{
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing.";
                TermText::new(spelling, definition)
            }];
            let term_texts = TERM_TEXTS;
            let language = Language::try_from(term_texts).unwrap();
            let directory_name = "harbor";
            let context = Context::from((directory_name, language));
            Book::from(&context)
        }
    }

    #[test]
    fn a_press_of_no_named_directory_assembles_the_book_of_every_context() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let directories: [PathBuf; 0] = [];

        let books = press.try_assemble_books(&directories).unwrap();

        let book_paths: Vec<&Path> = books.into_iter().map(Book::path).collect();
        let book_path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let book_path = Path::new(book_path_text);
        let expectation = [book_path];
        assert_eq!(book_paths, expectation);
    }

    #[test]
    fn the_directory_of_a_context_yields_the_book_of_that_context() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let directory_text = "crates/harbor";
        let directory = PathBuf::from(directory_text);
        let directories = [directory];

        let books = press.try_assemble_books(&directories).unwrap();

        let book_paths: Vec<&Path> = books.into_iter().map(Book::path).collect();
        let book_path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let book_path = Path::new(book_path_text);
        let expectation = [book_path];
        assert_eq!(book_paths, expectation);
    }

    #[test]
    fn a_directory_beneath_the_directory_of_a_context_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let directory_text = "crates/harbor/src";
        let directory = PathBuf::from(directory_text);
        let directories = [directory];

        let books = press.try_assemble_books(&directories);

        let error = books.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] `crates/harbor/src` is not the directory of a context.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_directory_no_context_admits_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let directory_text = "crates/nowhere";
        let directory = PathBuf::from(directory_text);
        let directories = [directory];

        let books = press.try_assemble_books(&directories);

        let error = books.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] `crates/nowhere` is not the directory of a context.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_context_of_no_term_yields_its_book() {
        let press = fixture::empty_press_factory();
        let directories: [PathBuf; 0] = [];

        let books = press.try_assemble_books(&directories).unwrap();

        let book_paths: Vec<&Path> = books.into_iter().map(Book::path).collect();
        let book_path_text = "crates/ship/docs/UBIQUITOUS_LANGUAGE.md";
        let book_path = Path::new(book_path_text);
        let expectation = [book_path];
        assert_eq!(book_paths, expectation);
    }

    #[test]
    fn a_named_directory_of_a_context_of_no_term_yields_its_book() {
        let press = fixture::empty_press_factory();
        let directory_text = "crates/ship";
        let directory = PathBuf::from(directory_text);
        let directories = [directory];

        let books = press.try_assemble_books(&directories).unwrap();

        let book_paths: Vec<&Path> = books.into_iter().map(Book::path).collect();
        let book_path_text = "crates/ship/docs/UBIQUITOUS_LANGUAGE.md";
        let book_path = Path::new(book_path_text);
        let expectation = [book_path];
        assert_eq!(book_paths, expectation);
    }

    #[test]
    fn the_directory_of_a_book_the_writer_detects_is_present() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let presence = press.try_detect_directory(&book).unwrap();

        let expectation = DirectoryPresence::Presence;
        assert_eq!(presence, expectation);
    }

    #[test]
    fn the_directory_of_a_book_the_writer_does_not_detect_is_absent() {
        let writer_adapter = fixture::WriterAdapter::Absence;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let presence = press.try_detect_directory(&book).unwrap();

        let expectation = DirectoryPresence::Absence;
        assert_eq!(presence, expectation);
    }

    #[test]
    fn a_directory_the_writer_cannot_detect_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::DetectDirectoryError;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let presence = press.try_detect_directory(&book);

        let error = presence.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] cannot check whether `crates/harbor/docs` is a directory: no sight.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_book_the_writer_accepts_is_written() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let outcome = press.try_write_book(&book);

        let error = outcome.err();
        let error_text = error.as_ref().map(ToString::to_string);
        let expectation = None;
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_book_whose_file_cannot_be_created_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::CreateFileError;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let outcome = press.try_write_book(&book);

        let error = outcome.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] cannot write `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: no file.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_book_whose_text_cannot_be_put_into_its_file_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::WriteError;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let outcome = press.try_write_book(&book);

        let error = outcome.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] cannot write `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: failed to write whole buffer.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_book_whose_file_cannot_be_flushed_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::FlushError;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let outcome = press.try_write_book(&book);

        let error = outcome.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] cannot write `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: stuck.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_book_whose_file_cannot_be_replaced_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::ReplaceError;
        let press = fixture::press_factory(writer_adapter);
        let book = fixture::book_factory();

        let outcome = press.try_write_book(&book);

        let error = outcome.unwrap_err();
        let error_text = error.to_string();
        let expectation = "[LANGUAGE] cannot write `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: no room.";
        assert_eq!(error_text, expectation);
    }

    mod helper {
        use super::*;

        pub(super) fn edition_texts(edition: Edition) -> Vec<String> {
            let entry_text = |book_entry: BookEntry| book_entry.to_string();
            edition.into_iter().map(entry_text).collect()
        }
    }

    #[test]
    fn a_book_edition_of_no_directory_publishes_every_book_whose_directory_is_present() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let directories = Vec::new();
        let book_edition = BookEdition::from(directories);

        let edition = press.dispatch_command(book_edition).unwrap();

        let edition_texts = helper::edition_texts(edition);
        let expectation = [
            "[UBIQUITOUS] `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md` now contains 1 term(s).",
        ];
        assert_eq!(edition_texts, expectation);
    }

    #[test]
    fn a_book_edition_of_no_directory_passes_over_a_book_whose_directory_is_absent() {
        let writer_adapter = fixture::WriterAdapter::Absence;
        let press = fixture::press_factory(writer_adapter);
        let directories = Vec::new();
        let book_edition = BookEdition::from(directories);

        let edition = press.dispatch_command(book_edition).unwrap();

        let edition_texts = helper::edition_texts(edition);
        let expectation = [
            "[UBIQUITOUS] skipped `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: its directory does not exist.",
        ];
        assert_eq!(edition_texts, expectation);
    }

    #[test]
    fn a_book_edition_of_no_directory_holds_the_error_of_a_directory_the_writer_cannot_detect() {
        let writer_adapter = fixture::WriterAdapter::DetectDirectoryError;
        let press = fixture::press_factory(writer_adapter);
        let directories = Vec::new();
        let book_edition = BookEdition::from(directories);

        let edition = press.dispatch_command(book_edition).unwrap();

        let edition_texts = helper::edition_texts(edition);
        let expectation = [
            "[LANGUAGE] cannot check whether `crates/harbor/docs` is a directory: no sight.",
        ];
        assert_eq!(edition_texts, expectation);
    }

    #[test]
    fn a_book_edition_of_a_directory_writes_its_book_without_a_look_at_its_directory() {
        let writer_adapter = fixture::WriterAdapter::Absence;
        let press = fixture::press_factory(writer_adapter);
        let directory_text = "crates/harbor";
        let directory = PathBuf::from(directory_text);
        let directories = vec![directory];
        let book_edition = BookEdition::from(directories);

        let edition = press.dispatch_command(book_edition).unwrap();

        let edition_texts = helper::edition_texts(edition);
        let expectation = [
            "[UBIQUITOUS] `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md` now contains 1 term(s).",
        ];
        assert_eq!(edition_texts, expectation);
    }

    #[test]
    fn a_book_edition_holds_the_error_of_a_book_the_writer_cannot_replace() {
        let writer_adapter = fixture::WriterAdapter::ReplaceError;
        let press = fixture::press_factory(writer_adapter);
        let directory_text = "crates/harbor";
        let directory = PathBuf::from(directory_text);
        let directories = vec![directory];
        let book_edition = BookEdition::from(directories);

        let edition = press.dispatch_command(book_edition).unwrap();

        let edition_texts = helper::edition_texts(edition);
        let expectation = [
            "[LANGUAGE] cannot write `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md`: no room.",
        ];
        assert_eq!(edition_texts, expectation);
    }

    #[test]
    fn a_book_edition_of_a_directory_no_context_admits_is_an_error() {
        let writer_adapter = fixture::WriterAdapter::Presence;
        let press = fixture::press_factory(writer_adapter);
        let directory_text = "crates/nowhere";
        let directory = PathBuf::from(directory_text);
        let directories = vec![directory];
        let book_edition = BookEdition::from(directories);

        let edition = press.dispatch_command(book_edition);

        let error_text = edition.unwrap_err().to_string();
        let expectation = "[LANGUAGE] `crates/nowhere` is not the directory of a context.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_vocabulary_whose_contexts_cannot_be_listed_makes_no_press() {
        let vocabulary_error_adapter = VocabularyErrorAdapter::Context;
        let writer_adapter = fixture::WriterAdapter::Presence;

        let press = Press::try_from((vocabulary_error_adapter, writer_adapter));

        let error_text = press.err().map(|error| error.to_string());
        let expectation_text = "[LANGUAGE] there is no Rust source under `contexts/treasure`.";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_vocabulary_whose_foreign_languages_cannot_be_listed_makes_no_press() {
        let vocabulary_error_adapter = VocabularyErrorAdapter::ForeignLanguage;
        let writer_adapter = fixture::WriterAdapter::Presence;

        let press = Press::try_from((vocabulary_error_adapter, writer_adapter));

        let error_text = press.err().map(|error| error.to_string());
        let expectation_text = "[LANGUAGE] there is no Rust source under `foreign_languages/anchor`.";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }
}
