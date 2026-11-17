use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterLockRecordFileSystemContext;

impl Layer for StackLockAdapterLockRecordFileSystemContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterLockRecordFileSystemContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/lock_record/file_system";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Lock", "Record", "File", "System",
            ];
            let definition = "The crate stack_lock_adapter_lock_record_file_system, the adapter of the port LockRecord of stack_lock_application over a file of JSON for every lock and a lease of flock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Record"];
            let definition = "The adapter of the port LockRecord, which keeps a file of JSON for every lock in the record directory and leases them through flock on a file of its own.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Record", "Port"];
            let definition = "The port LockRecord as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Configuration"];
            let definition = "The directory of the records of the locks.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The record configuration of the adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Directory"];
            let definition = "The directory of the records of the locks, /run/baton unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Lease"];
            let definition = "The exclusive lease of the records, an open file locked with flock, which the kernel ends when the file closes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lease", "Failure"];
            let definition = "The error of a lease of the records that failed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lease", "File"];
            let definition = "The open file the record lease locks with flock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lease", "Path"];
            let definition = "The path of the file of the lease, lease in the record directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lease", "Name"];
            let definition = "The name of the file of the lease.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Open", "Lease"];
            let definition = "To open the file of the lease and lock it exclusively, waiting for another lease to end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Document"];
            let definition = "The record of a lock as the file of JSON holds it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Document"];
            let definition = "A device file as the record of its lock holds it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Demand", "Document"];
            let definition = "A demand as the record of its lock holds it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["User", "Document"];
            let definition = "A console or a framebuffer an acquisition unbound, as the record of its lock holds it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Text"];
            let definition = "To write an error as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Device", "File"];
            let definition = "To turn the record of a device file into a device file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Input", "Error"];
            let definition = "To turn an error of a read into an error of the record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Path"];
            let definition = "The path of the record of a lock, its name with the extension json in the record directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Paths"];
            let definition = "The paths of the records in the record directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Text"];
            let definition = "The text of the record of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Name"];
            let definition = "The name of the file of the record of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Extension"];
            let definition = "The extension json of a record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Transfer", "Extension"];
            let definition = "The extension of the file a record is first written to before it replaces the record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Transfer", "Path"];
            let definition = "The path of the file a record is first written to before it replaces the record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Parse", "Lock"];
            let definition = "To read a lock from the text of its record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entries"];
            let definition = "The entries of the record directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry"];
            let definition = "An entry of the record directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Extension"];
            let definition = "The extension of the path of an entry of the record directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a file of the records.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Input"];
            let definition = "The error of a read of a record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "The error of a write of a record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Document"];
            let definition = "The error of a record that holds no lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Locks"];
            let definition = "The locks of every record.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterLockRecordFileSystemContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockAdapterLockRecordFileSystemContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterLockRecordFileSystemContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterLockRecordFileSystemContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_adapter_lock_record_file_system_context_is_a_term() {
        let context_type = StackLockAdapterLockRecordFileSystemContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
