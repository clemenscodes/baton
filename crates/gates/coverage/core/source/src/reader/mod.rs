use crate::Source;
use stack_domain_driven_design_core::port::DrivenPort;
use std::io::{self, Read};
use std::path::Path;

pub trait Reader: DrivenPort {
    fn try_read_source(&self, path: &Path) -> io::Result<Source>;

    fn try_open_report(&self, path: &Path) -> io::Result<impl Read + use<Self>>;
}
