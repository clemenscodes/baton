mod directory_presence;

pub use directory_presence::DirectoryPresence;

use stack_domain_driven_design_core::port::DrivenPort;
use std::io::{self, Write};
use std::path::Path;

pub trait Writer: DrivenPort {
    fn try_create_file(&self, path: &Path) -> io::Result<Box<dyn Write + Send>>;

    fn try_replace_file(&self, path: &Path) -> io::Result<()>;

    fn try_detect_directory(&self, directory: &Path) -> io::Result<DirectoryPresence>;
}
