use gates_coverage_core_invocation::{Invocation, Stream};
use stack_domain_driven_design_core::domain::DomainService;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::ffi::OsString;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Compiler;

impl Layer for Compiler {
    type Layer = DomainLayer;
}

impl DomainService for Compiler {}

impl Compiler {
    const PROGRAM_NAME: &'static str = "rustc";

    const LIBRARY_QUERY: [&'static str; 2] = ["--print", "target-libdir"];

    pub fn compose_library_invocation(&self) -> Invocation {
        let program = OsString::from(Self::PROGRAM_NAME);
        let arguments = Vec::from_iter(Self::LIBRARY_QUERY.map(OsString::from));
        let variables = Vec::new();
        let stream = Stream::Capture;
        Invocation::from((program, arguments, variables, stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_compiler_names_its_library_directory_on_its_output() {
        let compiler = Compiler;

        let invocation = compiler.compose_library_invocation();

        let program = OsString::from("rustc");
        let arguments = Vec::from_iter(["--print", "target-libdir"].map(OsString::from));
        let variables = Vec::new();
        let stream = Stream::Capture;
        let expectation = Invocation::from((program, arguments, variables, stream));
        assert_eq!(invocation, expectation);
    }
}
