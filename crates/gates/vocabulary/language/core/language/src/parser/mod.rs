use crate::source::{Source, SourceText};
use stack_domain_driven_design_core::port::DrivenPort;
use std::io;

pub trait Parser: DrivenPort {
    fn try_parse_source(&self, source_text: &SourceText) -> io::Result<Source>;
}
