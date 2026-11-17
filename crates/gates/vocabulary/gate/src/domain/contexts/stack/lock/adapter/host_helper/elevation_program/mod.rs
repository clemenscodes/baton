use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterHostHelperElevationProgramContext;

impl Layer for StackLockAdapterHostHelperElevationProgramContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterHostHelperElevationProgramContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/host_helper/elevation_program";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack",
                "Lock",
                "Adapter",
                "Host",
                "Helper",
                "Elevation",
                "Program",
            ];
            let definition = "The crate stack_lock_adapter_host_helper_elevation_program, the adapter of the port HostHelper of stack_lock_application, which runs baton_host through an elevation program such as sudo.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Helper"];
            let definition = "The adapter of the port HostHelper, which runs baton_host with a request through the elevation program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Helper", "Port"];
            let definition = "The port HostHelper as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Helper", "Configuration"];
            let definition = "The elevation program, its arguments and the path of baton_host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The helper configuration of the adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Elevation", "Program"];
            let definition = "The program that runs baton_host as root, sudo unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Elevation", "Arguments"];
            let definition = "The arguments of the elevation program in front of baton_host, -n for sudo so it never asks for a password.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Program"];
            let definition = "The path of baton_host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The spawner of stack_process_core_invocation that starts the elevation program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner", "Type"];
            let definition = "The type of the spawner a host helper holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Request", "Arguments"];
            let definition = "The request and its options baton_host reads.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Request", "Name"];
            let definition = "The name of the request of baton_host as a text in an error, such as acquire.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Spawn", "Request"];
            let definition = "To run baton_host with a request through the elevation program and answer its output.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Request"];
            let definition = "The error of a request of baton_host that ended without success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Failure"];
            let definition = "The error of an elevation program that could not be started.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status"];
            let definition = "The exit status of baton_host as the operating system reports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "The standard output of baton_host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output", "Text"];
            let definition = "The standard output of baton_host as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Arguments"];
            let definition = "The arguments of the elevation program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Invocation"];
            let definition = "The invocation of the elevation program.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterHostHelperElevationProgramContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockAdapterHostHelperElevationProgramContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterHostHelperElevationProgramContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterHostHelperElevationProgramContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_stack_lock_adapter_host_helper_elevation_program_context_is_a_term()
    {
        let context_type = StackLockAdapterHostHelperElevationProgramContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
