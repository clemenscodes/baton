use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct ProductsBatonPresentationCommandLineInterfaceContext;

impl Layer for ProductsBatonPresentationCommandLineInterfaceContext {
    type Layer = DomainLayer;
}

impl ProductsBatonPresentationCommandLineInterfaceContext {
    const DIRECTORY_NAME: &'static str = "products/baton/presentation/command_line_interface";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Products",
                "Baton",
                "Presentation",
                "Command",
                "Line",
                "Interface",
            ];
            let definition = "The crate products_baton_presentation_command_line_interface, the program baton without a privilege, which runs a program inside the lock of a GPU and prints the state of every lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Line"];
            let definition = "The command line of the program baton, which composes the lock guard from its arguments and answers the request.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Arguments"];
            let definition = "The arguments of the program baton, its options and its request.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Request"];
            let definition = "The request of the program baton, run or status.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Run"];
            let definition = "The request that runs a program inside the lock of a GPU, the guard of the mutex.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status"];
            let definition = "The request that prints the state of every lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock"];
            let definition = "The name of the lock a run acquires.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application"];
            let definition = "The name of the eligible application a run acquires the lock for.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Deadline"];
            let definition = "The time in milliseconds a run waits for the GPU at most.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Force"];
            let definition = "Whether a run ends the users of the GPU instead of waiting for them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Passthrough"];
            let definition = "Whether a run hands the GPU to vfio-pci for a guest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program"];
            let definition = "The program a run starts inside the lock, with its arguments, after --.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Elevation", "Program"];
            let definition = "The program that runs baton_host as root, sudo unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Elevation", "Argument"];
            let definition = "An argument of the elevation program in front of baton_host, -n by default so sudo never asks for a password.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Program"];
            let definition = "The path of baton_host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Guard", "Adapters"];
            let definition = "The host helper and the program runner the lock guard drives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Helper", "Spawner"];
            let definition = "The spawner the host helper starts the elevation program with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Runner", "Spawner"];
            let definition = "The spawner the program runner starts the program of a run with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Failure", "Code"];
            let definition = "The exit code 1 of a run whose program ended with a code that is no byte.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit", "Byte"];
            let definition = "The exit code of the program of a run as the byte the program baton ends with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Respond", "Exit", "Code"];
            let definition = "To answer the request of the arguments and to answer the exit code of the program baton.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Deadline"];
            let definition = "The deadline of a run, where it names one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawners"];
            let definition = "The two spawners of the host helper and of the program runner.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<ProductsBatonPresentationCommandLineInterfaceContext> for Context {
    type Error = Error;

    fn try_from(
        _: ProductsBatonPresentationCommandLineInterfaceContext,
    ) -> Result<Self, Self::Error> {
        let directory_name = ProductsBatonPresentationCommandLineInterfaceContext::DIRECTORY_NAME;
        let term_texts = ProductsBatonPresentationCommandLineInterfaceContext::TERM_TEXTS;
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
    fn every_definition_of_the_products_baton_presentation_command_line_interface_context_is_a_term()
     {
        let context_type = ProductsBatonPresentationCommandLineInterfaceContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
