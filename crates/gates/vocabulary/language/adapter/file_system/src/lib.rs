mod error;

use crate::error::Error;
use gates_vocabulary_language_core_books::writer::{DirectoryPresence, Writer};
use gates_vocabulary_language_core_language::reader::Reader;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::ffi::OsStr;
use std::fs::{DirEntry, File, FileType, metadata, read_dir, remove_file, rename};
use std::io::{self, ErrorKind, Read, Write};
use std::path::{Path, PathBuf};
use std::process::id;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct FileSystem;

impl Layer for FileSystem {
    type Layer = InfrastructureLayer;
}

impl Adapter for FileSystem {}

impl Port for FileSystem {}

impl DrivenPort for FileSystem {}

impl FileSystem {
    const SUFFIX_SEPARATOR: char = '.';

    const SOURCE_EXTENSION: &'static str = "rs";

    fn compose_temporary_path(&self, path: &Path) -> PathBuf {
        let mut temporary_path_text = path.as_os_str().to_os_string();
        let suffix_separator = Self::SUFFIX_SEPARATOR;
        let process_identifier = id();
        let suffix = format!("{suffix_separator}{process_identifier}");
        temporary_path_text.push(suffix);
        PathBuf::from(temporary_path_text)
    }

    fn try_walk_directory(&self, directory: &Path) -> io::Result<Vec<PathBuf>> {
        let source_extension = OsStr::new(Self::SOURCE_EXTENSION);
        let directory_criterion = |file_type: FileType| file_type.is_dir();
        let mut source_paths = Vec::new();
        let mut pending_directories = vec![directory.to_path_buf()];
        while let Some(pending_directory) = pending_directories.pop() {
            let directory_entries: Vec<DirEntry> = read_dir(pending_directory)
                .and_then(Iterator::collect)?;
            for directory_entry in directory_entries {
                let path = directory_entry.path();
                if directory_entry.file_type().is_ok_and(directory_criterion) {
                    pending_directories.push(path);
                } else if path.extension() == Some(source_extension) {
                    source_paths.push(path);
                }
            }
        }
        source_paths.sort();
        Ok(source_paths)
    }
}

impl Reader for FileSystem {
    fn try_open_file(&self, path: &Path) -> io::Result<Box<dyn Read + Send>> {
        let file = File::open(path)?;
        let reader: Box<dyn Read + Send> = Box::new(file);
        Ok(reader)
    }

    fn try_list_sources(&self, path: &Path) -> io::Result<Vec<PathBuf>> {
        match path.is_dir() {
            true => self.try_walk_directory(path),
            false => {
                let source_path = path.to_path_buf();
                Ok(vec![source_path])
            }
        }
    }
}

impl Writer for FileSystem {
    fn try_create_file(&self, path: &Path) -> io::Result<Box<dyn Write + Send>> {
        let temporary_path = self.compose_temporary_path(path);
        let file = File::create(temporary_path)?;
        let writer: Box<dyn Write + Send> = Box::new(file);
        Ok(writer)
    }

    fn try_replace_file(&self, path: &Path) -> io::Result<()> {
        let temporary_path = self.compose_temporary_path(path);
        let Err(replacement_error) = rename(&temporary_path, path) else {
            return Ok(());
        };
        let removal = remove_file(&temporary_path);
        match removal {
            Ok(()) => Err(replacement_error),
            Err(error) => {
                let removal_error = Error::Removal {
                    replacement_error,
                    temporary_path,
                    error,
                };
                let file_system_error = io::Error::other(removal_error);
                Err(file_system_error)
            }
        }
    }

    fn try_detect_directory(&self, directory: &Path) -> io::Result<DirectoryPresence> {
        let presence = DirectoryPresence::Presence;
        let absence = DirectoryPresence::Absence;
        match metadata(directory) {
            Ok(directory_metadata) => match directory_metadata.is_dir() {
                true => Ok(presence),
                false => Ok(absence),
            },
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(absence),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::{create_dir_all, read_to_string, remove_dir_all};

    mod fixture {
        use super::*;

        pub(super) fn directory_factory(directory_name: &str) -> PathBuf {
            let root = temp_dir();
            let directory = root.join(directory_name);
            create_dir_all(&directory).unwrap();
            remove_dir_all(&directory).unwrap();
            create_dir_all(&directory).unwrap();
            directory
        }

        pub(super) fn file_factory(path: &Path) {
            let file_system = FileSystem;
            let mut file = file_system.try_create_file(path).unwrap();
            let file_bytes: &[u8] = b"struct Treasure;";
            file.write_all(file_bytes).unwrap();
            file_system.try_replace_file(path).unwrap();
        }
    }

    #[test]
    fn a_file_that_is_here_is_opened_for_its_text() {
        let file_system = FileSystem;
        let path_text = "Cargo.toml";
        let path = Path::new(path_text);

        let file = file_system.try_open_file(path).unwrap();

        let text = io::read_to_string(file).unwrap();
        let first_line = text.lines().next();
        let expectation_text = "[package]";
        let expectation = Some(expectation_text);
        assert_eq!(first_line, expectation);
    }

    #[test]
    fn a_file_that_is_not_here_is_an_error() {
        let file_system = FileSystem;
        let path_text = "nowhere/mod.rs";
        let path = Path::new(path_text);

        let file = file_system.try_open_file(path);

        let error = file.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::NotFound;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_directory_lists_every_rust_source_beneath_it_in_their_order() {
        let file_system = FileSystem;
        let directory_text = "language-source-listing";
        let directory = fixture::directory_factory(directory_text);
        let crate_directory_text = "harbor";
        let crate_directory = directory.join(crate_directory_text);
        create_dir_all(&crate_directory).unwrap();
        let root_source_path_text = "ship.rs";
        let root_source_path = directory.join(root_source_path_text);
        let crate_source_path_text = "lib.rs";
        let crate_source_path = crate_directory.join(crate_source_path_text);
        let manifest_path_text = "Cargo.toml";
        let manifest_path = directory.join(manifest_path_text);
        fixture::file_factory(&root_source_path);
        fixture::file_factory(&crate_source_path);
        fixture::file_factory(&manifest_path);

        let source_paths = file_system.try_list_sources(&directory);

        remove_dir_all(&directory).unwrap();
        let expectation = vec![crate_source_path, root_source_path];
        assert_eq!(source_paths.unwrap(), expectation);
    }

    #[test]
    fn a_path_that_is_no_directory_lists_itself() {
        let file_system = FileSystem;
        let path_text = "nowhere/mod.rs";
        let path = Path::new(path_text);

        let source_paths = file_system.try_list_sources(path).unwrap();

        let source_path = path.to_path_buf();
        let expectation = vec![source_path];
        assert_eq!(source_paths, expectation);
    }

    #[test]
    fn a_walk_from_a_file_is_an_error() {
        let file_system = FileSystem;
        let path_text = "Cargo.toml";
        let path = Path::new(path_text);

        let source_paths = file_system.try_walk_directory(path);

        let error = source_paths.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::NotADirectory;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_file_whose_directory_is_not_here_is_an_error() {
        let file_system = FileSystem;
        let path_text = "nowhere/mod.rs";
        let path = Path::new(path_text);

        let file = file_system.try_create_file(path);

        let error = file.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::NotFound;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_file_the_file_system_replaces_holds_the_text_put_into_it() {
        let file_system = FileSystem;
        let directory_text = "language-replacement";
        let directory = fixture::directory_factory(directory_text);
        let path_text = "mod.rs";
        let path = directory.join(path_text);
        let mut file = file_system.try_create_file(&path).unwrap();
        let file_bytes: &[u8] = b"struct Treasure;";
        file.write_all(file_bytes).unwrap();

        let replacement = file_system.try_replace_file(&path);

        let text = read_to_string(&path).unwrap();
        remove_dir_all(&directory).unwrap();
        let error = replacement.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let expectation = None;
        assert_eq!(error_kind, expectation);
        let text_expectation = "struct Treasure;";
        assert_eq!(text, text_expectation);
    }

    #[test]
    fn a_file_the_file_system_replaces_twice_holds_what_came_last() {
        let file_system = FileSystem;
        let directory_text = "language-second-replacement";
        let directory = fixture::directory_factory(directory_text);
        let path_text = "mod.rs";
        let path = directory.join(path_text);
        fixture::file_factory(&path);
        let mut file = file_system.try_create_file(&path).unwrap();
        let file_bytes: &[u8] = b"struct Chest;";
        file.write_all(file_bytes).unwrap();

        let replacement = file_system.try_replace_file(&path);

        let text = read_to_string(&path).unwrap();
        remove_dir_all(&directory).unwrap();
        let error = replacement.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let expectation = None;
        assert_eq!(error_kind, expectation);
        let text_expectation = "struct Chest;";
        assert_eq!(text, text_expectation);
    }

    #[test]
    fn the_temporary_path_of_a_file_is_beside_the_file() {
        let file_system = FileSystem;
        let path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let path = Path::new(path_text);

        let temporary_path = file_system.compose_temporary_path(path);

        let temporary_path_parent = temporary_path.parent();
        let directory = path.parent();
        assert_eq!(temporary_path_parent, directory);
    }

    #[test]
    fn a_directory_in_the_place_of_a_temporary_file_is_an_error() {
        let file_system = FileSystem;
        let directory_text = "language-blocked-creation";
        let directory = fixture::directory_factory(directory_text);
        let path_text = "mod.rs";
        let path = directory.join(path_text);
        fixture::file_factory(&path);
        let temporary_path = file_system.compose_temporary_path(&path);
        create_dir_all(&temporary_path).unwrap();

        let file = file_system.try_create_file(&path);

        let text = read_to_string(&path).unwrap();
        remove_dir_all(&directory).unwrap();
        let error = file.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::IsADirectory;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
        let text_expectation = "struct Treasure;";
        assert_eq!(text, text_expectation);
    }

    #[test]
    fn a_replacement_over_a_directory_is_an_error() {
        let file_system = FileSystem;
        let directory_text = "language-blocked-replacement";
        let directory = fixture::directory_factory(directory_text);
        let path_text = "mod.rs";
        let path = directory.join(path_text);
        create_dir_all(&path).unwrap();
        let mut file = file_system.try_create_file(&path).unwrap();
        let file_bytes: &[u8] = b"struct Chest;";
        file.write_all(file_bytes).unwrap();

        let replacement = file_system.try_replace_file(&path);

        let entry_count = read_dir(&directory).unwrap().count();
        remove_dir_all(&directory).unwrap();
        let error = replacement.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::IsADirectory;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
        let count_expectation: usize = 1;
        assert_eq!(entry_count, count_expectation);
    }

    #[test]
    fn a_replacement_whose_temporary_file_cannot_be_removed_is_an_error() {
        let file_system = FileSystem;
        let path_text = "nowhere/mod.rs";
        let path = Path::new(path_text);

        let replacement = file_system.try_replace_file(path);

        let error = replacement.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let temporary_path = file_system.compose_temporary_path(path);
        let temporary_path_text = temporary_path.display();
        let absence_text = "No such file or directory (os error 2)";
        let text_expectation = format!(
            "{absence_text}, and the temporary file `{temporary_path_text}` remains because it cannot be removed: {absence_text}"
        );
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_directory_that_is_here_is_present() {
        let file_system = FileSystem;
        let directory_text = "src";
        let directory = Path::new(directory_text);

        let presence = file_system.try_detect_directory(directory).unwrap();

        let expectation = DirectoryPresence::Presence;
        assert_eq!(presence, expectation);
    }

    #[test]
    fn a_file_is_no_directory_and_so_the_directory_is_absent() {
        let file_system = FileSystem;
        let directory_text = "Cargo.toml";
        let directory = Path::new(directory_text);

        let presence = file_system.try_detect_directory(directory).unwrap();

        let expectation = DirectoryPresence::Absence;
        assert_eq!(presence, expectation);
    }

    #[test]
    fn a_directory_that_is_not_here_is_absent() {
        let file_system = FileSystem;
        let directory_text = "nowhere/docs";
        let directory = Path::new(directory_text);

        let presence = file_system.try_detect_directory(directory).unwrap();

        let expectation = DirectoryPresence::Absence;
        assert_eq!(presence, expectation);
    }

    #[test]
    fn a_directory_below_a_file_is_an_error() {
        let file_system = FileSystem;
        let directory_text = "Cargo.toml/docs";
        let directory = Path::new(directory_text);

        let presence = file_system.try_detect_directory(directory);

        let error = presence.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::NotADirectory;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }
}
