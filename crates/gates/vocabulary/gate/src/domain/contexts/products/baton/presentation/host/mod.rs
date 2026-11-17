use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct ProductsBatonPresentationHostContext;

impl Layer for ProductsBatonPresentationHostContext {
    type Layer = DomainLayer;
}

impl ProductsBatonPresentationHostContext {
    const DIRECTORY_NAME: &'static str = "products/baton/presentation/host";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Products", "Baton", "Presentation", "Host"];
            let definition = "The crate products_baton_presentation_host, the helper baton_host run as root, which administers the locks of the GPUs and answers the requests of baton.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Request"];
            let definition = "A request of baton_host, such as enclose or acquire, with its arguments.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Users"];
            let definition = "The request of baton_host that names every user of the GPU of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application"];
            let definition = "The application a request of baton_host names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Force"];
            let definition = "The flag of an acquisition that ends the users of a GPU instead of waiting for them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Passthrough"];
            let definition = "The flag of an acquisition that hands the GPU to vfio-pci for a guest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process"];
            let definition = "The process of an acquisition, which moves into the cgroup of its application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Adapters"];
            let definition = "The adapters of every port of the administration, which baton_host composes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Respond", "Exit", "Code"];
            let definition = "To respond to a request of baton_host and answer its exit code.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<ProductsBatonPresentationHostContext> for Context {
    type Error = Error;

    fn try_from(_: ProductsBatonPresentationHostContext) -> Result<Self, Self::Error> {
        let directory_name = ProductsBatonPresentationHostContext::DIRECTORY_NAME;
        let term_texts = ProductsBatonPresentationHostContext::TERM_TEXTS;
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
    fn every_definition_of_the_products_baton_presentation_host_context_is_a_term() {
        let context_type = ProductsBatonPresentationHostContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
