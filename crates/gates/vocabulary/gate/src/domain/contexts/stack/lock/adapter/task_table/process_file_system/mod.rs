use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterTaskTableProcessFileSystemContext;

impl Layer for StackLockAdapterTaskTableProcessFileSystemContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterTaskTableProcessFileSystemContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/task_table/process_file_system";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Task", "Table", "Process", "File", "System",
            ];
            let definition = "The crate stack_lock_adapter_task_table_process_file_system, the adapter of the port TaskTable of stack_lock_application over the descriptors, the commands and the cgroups under /proc and the signals of rustix.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Task", "Table"];
            let definition = "The adapter of the port TaskTable, which reads the descriptors, the commands and the cgroups of the processes under /proc and sends the signals through a signal call.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Task", "Table", "Port"];
            let definition = "The port TaskTable as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Task", "Configuration"];
            let definition = "The directory of the processes of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The task configuration of the adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Task", "Directory"];
            let definition = "The directory of the processes of the host, /proc unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process", "Path"];
            let definition = "The directory of one process under the directory of the processes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process", "Identifiers"];
            let definition = "The identifiers of the processes under the directory of the processes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Descriptor", "Path"];
            let definition = "The directory fd of a process, which holds a link for every open descriptor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Descriptor", "Name"];
            let definition = "The name fd of the directory of the descriptors of a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Name"];
            let definition = "The name comm of the file of the command of a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Name"];
            let definition = "The name cgroup of the file of the cgroup of a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Prefix"];
            let definition = "The prefix 0:: of the line of the unified hierarchy in the file cgroup of a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Text"];
            let definition = "The line of the file cgroup of a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Holder"];
            let definition = "To tell whether a process holds one of the device files through an open descriptor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Holder", "Presence"];
            let definition = "Whether a process holds one of the device files.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Device", "Kind"];
            let definition = "The device kind of the file an open descriptor points to, where it is a character device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Identity"];
            let definition = "The identity of a device file, rdev, which holds its major and its minor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Type"];
            let definition = "The type of the file an open descriptor points to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Text"];
            let definition = "To read the first line of a file of a process, or nothing where it has none.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Call"];
            let definition = "The seam of the signals the adapter sends, so a test sends no signal to a real process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Call", "Type"];
            let definition = "The type of the signal call a task table holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Kind"];
            let definition = "Which signal a signal call sends, the termination or the kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Termination"];
            let definition = "The signal of termination, SIGTERM, which asks a process to end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Kill"];
            let definition = "The signal SIGKILL, which ends a process the kernel will not let survive.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["System", "Signal"];
            let definition = "The signal call over the system call kill of rustix.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Target"];
            let definition = "The process identifier rustix sends a signal to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Failure"];
            let definition = "The error of a signal the kernel refused.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Send", "Signal"];
            let definition = "To send a signal to a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Signal", "Process"];
            let definition = "To send a signal to a process and take a process that already ended as no error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence", "Code"];
            let definition = "The error number 3 of the kernel, ESRCH, for a process that does not exist.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Scan", "Failure"];
            let definition = "The error of a directory of the processes that could not be read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entries"];
            let definition = "The entries of a directory under /proc.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry"];
            let definition = "An entry of a directory under /proc.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name", "Text"];
            let definition = "The name of an entry under /proc as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Metadata"];
            let definition = "The metadata of the file an open descriptor points to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a file of a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Kernel", "Signal"];
            let definition = "The signal of rustix a system signal sends.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Line"];
            let definition = "One line of the file cgroup of a process, of which the one of the unified hierarchy begins with 0::.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterTaskTableProcessFileSystemContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockAdapterTaskTableProcessFileSystemContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterTaskTableProcessFileSystemContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterTaskTableProcessFileSystemContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_adapter_task_table_process_file_system_context_is_a_term()
    {
        let context_type = StackLockAdapterTaskTableProcessFileSystemContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
