use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::fs::File;

#[derive(Debug)]
pub struct RecordLease {
    lease_file: File,
}

impl Layer for RecordLease {
    type Layer = InfrastructureLayer;
}

impl RecordLease {
    pub fn lease_file(&self) -> &File {
        &self.lease_file
    }
}

impl From<File> for RecordLease {
    fn from(lease_file: File) -> Self {
        Self { lease_file }
    }
}
