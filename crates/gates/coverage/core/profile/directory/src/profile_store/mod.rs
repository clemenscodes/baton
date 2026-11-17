use crate::ProfileDirectory;
use gates_coverage_core_report::ReportFile;
use stack_domain_driven_design_core::port::DrivenPort;
use std::io;
use std::path::{Path, PathBuf};

pub trait ProfileStore: DrivenPort {
    fn try_create_profile_directory(&self, profile_root: &Path) -> io::Result<ProfileDirectory>;

    fn try_list_profiles(&self, profile_directory: &ProfileDirectory) -> io::Result<Vec<PathBuf>>;

    fn try_remove_profile_directory(&self, profile_directory: &ProfileDirectory) -> io::Result<()>;

    fn try_write_report_file(&self, report_file: &ReportFile) -> io::Result<()>;
}
