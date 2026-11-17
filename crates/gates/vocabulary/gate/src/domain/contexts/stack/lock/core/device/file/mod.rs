use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockCoreDeviceFileContext;

impl Layer for StackLockCoreDeviceFileContext {
    type Layer = DomainLayer;
}

impl StackLockCoreDeviceFileContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/core/device/file";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Device", "File"];
            let definition = "The crate stack_lock_core_device_file, a device file of a GPU, its kind, its major and its minor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "File"];
            let definition = "A file under /dev of a GPU, named by its device kind, its major and its minor, which the device program locks.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Kind"];
            let definition = "Whether a device file is a character device or a block device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Character", "Device"];
            let definition = "A device file read and written as a stream, as every device file of a GPU is.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Block", "Device"];
            let definition = "A device file read and written in blocks, such as a disk.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Major"];
            let definition = "The major number of a device file, which names its driver, such as 195 for the files of nvidia.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Minor"];
            let definition = "The minor number of a device file, which names the device of its driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "The text of a device file or of a device kind.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockCoreDeviceFileContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockCoreDeviceFileContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockCoreDeviceFileContext::DIRECTORY_NAME;
        let term_texts = StackLockCoreDeviceFileContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_core_device_file_context_is_a_term() {
        let context_type = StackLockCoreDeviceFileContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
