use gates_coverage_core_profile_directory::ProfileDirectory;
use gates_coverage_core_profile_directory::ProfileStore;
use gates_coverage_core_report::ReportFile;
use gates_coverage_core_source::Reader;
use gates_coverage_core_source::Source;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::ffi::OsStr;
use std::fs::{self, DirEntry, File};
use std::io::{self, ErrorKind, Read};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct FileSystem;

impl Layer for FileSystem {
    type Layer = InfrastructureLayer;
}

impl Adapter for FileSystem {}

impl Port for FileSystem {}

impl DrivenPort for FileSystem {}

impl Reader for FileSystem {
    fn try_read_source(&self, path: &Path) -> io::Result<Source> {
        let text = fs::read_to_string(path)?;
        let source_path = path.to_path_buf();
        let source = Source::from((source_path, text));
        Ok(source)
    }

    fn try_open_report(&self, path: &Path) -> io::Result<impl Read + use<>> {
        File::open(path)
    }
}

impl ProfileStore for FileSystem {
    fn try_create_profile_directory(&self, profile_root: &Path) -> io::Result<ProfileDirectory> {
        let profile_directory_name = process::id().to_string();
        let path = profile_root.join(profile_directory_name);
        match fs::remove_dir_all(&path) {
            Err(error) if error.kind() != ErrorKind::NotFound => return Err(error),
            Err(_) | Ok(()) => {}
        }
        fs::create_dir_all(&path)?;
        let profile_directory = ProfileDirectory::from(path);
        Ok(profile_directory)
    }

    fn try_list_profiles(&self, profile_directory: &ProfileDirectory) -> io::Result<Vec<PathBuf>> {
        let directory_entries = fs::read_dir(profile_directory.path())?;
        self.try_select_profiles(directory_entries)
    }

    fn try_remove_profile_directory(&self, profile_directory: &ProfileDirectory) -> io::Result<()> {
        fs::remove_dir_all(profile_directory.path())
    }

    fn try_write_report_file(&self, report_file: &ReportFile) -> io::Result<()> {
        let report_path = report_file.path();
        let report_directory = report_path.with_file_name("");
        fs::create_dir_all(report_directory)?;
        fs::write(report_path, report_file.export())
    }
}

impl FileSystem {
    const PROFILE_EXTENSION: &'static str = "profraw";

    fn try_select_profiles(
        &self,
        directory_entries: impl Iterator<Item = io::Result<DirEntry>>,
    ) -> io::Result<Vec<PathBuf>> {
        let profile_extension = OsStr::new(Self::PROFILE_EXTENSION);
        let mut profile_paths = Vec::new();
        for directory_entry in directory_entries {
            let path = directory_entry?.path();
            if path.extension() == Some(profile_extension) {
                profile_paths.push(path);
            }
        }
        profile_paths.sort();
        Ok(profile_paths)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn a_file_that_is_here_is_read_as_a_source() {
        let path_text = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        let path = Path::new(path_text);
        let file_system = FileSystem;

        let source = file_system.try_read_source(path).unwrap();

        let source_path = path.to_path_buf();
        let text = fs::read_to_string(path).unwrap();
        let expectation = Source::from((source_path, text));
        assert_eq!(source, expectation);
    }

    #[test]
    fn a_file_that_is_not_here_is_an_error() {
        let path_text = "nowhere/Cargo.toml";
        let path = Path::new(path_text);
        let file_system = FileSystem;

        let source = file_system.try_read_source(path);

        let error_kind = source.unwrap_err().kind();
        let expectation = io::ErrorKind::NotFound;
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_report_that_is_here_is_opened_with_its_bytes() {
        let path_text = concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml");
        let path = Path::new(path_text);
        let file_system = FileSystem;
        let mut report_text = String::new();

        let mut report_file = file_system.try_open_report(path).unwrap();
        report_file.read_to_string(&mut report_text).unwrap();

        let expectation = fs::read_to_string(path).unwrap();
        assert_eq!(report_text, expectation);
    }

    #[test]
    fn a_report_that_is_not_here_is_an_error() {
        let path_text = "nowhere/report.json";
        let path = Path::new(path_text);
        let file_system = FileSystem;

        let report_file = file_system.try_open_report(path);

        let error_kind = report_file.err().map(|error| error.kind());
        let expectation = Some(io::ErrorKind::NotFound);
        assert_eq!(error_kind, expectation);
    }

    mod fixture {
        use super::*;

        pub(crate) fn profile_root_factory(test_name: &str) -> PathBuf {
            let profile_directory_name = process::id().to_string();
            let profile_root = std::env::temp_dir()
                .join("coverage-tests")
                .join(profile_directory_name)
                .join(test_name);
            fs::create_dir_all(&profile_root).unwrap();
            profile_root
        }
    }

    #[test]
    fn a_profile_directory_is_an_empty_directory_named_for_the_process_under_its_root() {
        let profile_root = fixture::profile_root_factory("empty");
        let file_system = FileSystem;

        let profile_directory = file_system
            .try_create_profile_directory(&profile_root)
            .unwrap();

        let profile_directory_name = process::id().to_string();
        let path_expectation = profile_root.join(profile_directory_name);
        let entry_count = fs::read_dir(profile_directory.path()).unwrap().count();
        assert_eq!(profile_directory.path(), path_expectation);
        assert_eq!(entry_count, 0);
    }

    #[test]
    fn a_profile_directory_left_by_an_earlier_process_of_the_same_number_starts_empty() {
        let profile_root = fixture::profile_root_factory("stale");
        let profile_directory_name = process::id().to_string();
        let profile_directory_path = profile_root.join(profile_directory_name);
        fs::create_dir_all(&profile_directory_path).unwrap();
        fs::write(profile_directory_path.join("1-2.profraw"), b"stale").unwrap();
        let file_system = FileSystem;

        let profile_directory = file_system
            .try_create_profile_directory(&profile_root)
            .unwrap();

        let entry_count = fs::read_dir(profile_directory.path()).unwrap().count();
        assert_eq!(entry_count, 0);
    }

    #[test]
    fn a_profile_directory_whose_place_holds_a_file_is_an_error() {
        let profile_root = fixture::profile_root_factory("file");
        let profile_directory_name = process::id().to_string();
        fs::write(profile_root.join(profile_directory_name), b"no directory").unwrap();
        let file_system = FileSystem;

        let profile_directory = file_system.try_create_profile_directory(&profile_root);

        let error_kind = profile_directory.unwrap_err().kind();
        assert_eq!(error_kind, ErrorKind::NotADirectory);
    }

    #[test]
    fn the_profiles_of_a_directory_are_its_raw_profiles_in_their_order() {
        let profile_root = fixture::profile_root_factory("listing");
        let file_system = FileSystem;
        let profile_directory = file_system
            .try_create_profile_directory(&profile_root)
            .unwrap();
        let path = profile_directory.path();
        fs::write(path.join("2-9.profraw"), b"second").unwrap();
        fs::write(path.join("1-9.profraw"), b"first").unwrap();
        fs::write(path.join("coverage.profdata"), b"merged").unwrap();

        let profile_paths = file_system.try_list_profiles(&profile_directory).unwrap();

        let first_expectation = path.join("1-9.profraw");
        let last_expectation = path.join("2-9.profraw");
        assert_eq!(profile_paths, [first_expectation, last_expectation]);
    }

    #[test]
    fn a_removed_profile_directory_is_gone_with_its_profiles() {
        let profile_root = fixture::profile_root_factory("removal");
        let file_system = FileSystem;
        let profile_directory = file_system
            .try_create_profile_directory(&profile_root)
            .unwrap();
        fs::write(profile_directory.path().join("1-9.profraw"), b"first").unwrap();

        file_system
            .try_remove_profile_directory(&profile_directory)
            .unwrap();

        assert!(!profile_directory.path().exists());
    }

    #[test]
    fn a_report_file_is_written_into_its_directory_with_its_export() {
        let profile_root = fixture::profile_root_factory("report");
        let report_path = profile_root.join("reports").join("harbor.json");
        let export = b"{\"data\":[]}".to_vec();
        let report_file = ReportFile::from((report_path, export));
        let file_system = FileSystem;

        file_system.try_write_report_file(&report_file).unwrap();

        let report_export = fs::read(report_file.path()).unwrap();
        assert_eq!(report_export, report_file.export());
    }

    #[test]
    fn a_profile_directory_whose_root_cannot_be_written_is_an_error() {
        let profile_root = fixture::profile_root_factory("locked");
        let permissions = fs::Permissions::from_mode(0o555);
        fs::set_permissions(&profile_root, permissions).unwrap();
        let file_system = FileSystem;

        let profile_directory = file_system.try_create_profile_directory(&profile_root);

        let error_kind = profile_directory.unwrap_err().kind();
        assert_eq!(error_kind, ErrorKind::PermissionDenied);
    }

    #[test]
    fn the_profiles_of_a_directory_that_is_not_here_are_an_error() {
        let path = PathBuf::from("nowhere/profiles");
        let profile_directory = ProfileDirectory::from(path);
        let file_system = FileSystem;

        let profile_paths = file_system.try_list_profiles(&profile_directory);

        let error_kind = profile_paths.unwrap_err().kind();
        assert_eq!(error_kind, ErrorKind::NotFound);
    }

    #[test]
    fn an_entry_of_a_directory_that_cannot_be_read_is_an_error() {
        let entry_error = io::Error::from(ErrorKind::PermissionDenied);
        let directory_entries: [io::Result<DirEntry>; 1] = [Err(entry_error)];
        let file_system = FileSystem;

        let profile_paths = file_system.try_select_profiles(directory_entries.into_iter());

        let error_kind = profile_paths.unwrap_err().kind();
        assert_eq!(error_kind, ErrorKind::PermissionDenied);
    }

    #[test]
    fn a_report_file_whose_directory_cannot_be_created_is_an_error() {
        let profile_root = fixture::profile_root_factory("blocked");
        let file_path = profile_root.join("reports");
        fs::write(&file_path, b"no directory").unwrap();
        let report_path = file_path.join("coverage").join("harbor.json");
        let export = b"{\"data\":[]}".to_vec();
        let report_file = ReportFile::from((report_path, export));
        let file_system = FileSystem;

        let report_result = file_system.try_write_report_file(&report_file);

        let error_kind = report_result.unwrap_err().kind();
        assert_eq!(error_kind, ErrorKind::NotADirectory);
    }
}
