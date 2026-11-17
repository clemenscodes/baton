use stack_domain_driven_design_core::port::DrivenPort;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub trait Reader: DrivenPort {
    fn try_open_file(&self, path: &Path) -> io::Result<Box<dyn Read + Send>>;

    fn try_list_sources(&self, path: &Path) -> io::Result<Vec<PathBuf>>;
}
