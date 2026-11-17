use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockApplicationContext;

impl Layer for StackLockApplicationContext {
    type Layer = DomainLayer;
}

impl StackLockApplicationContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/application";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Application"];
            let definition = "The crate stack_lock_application, the application of the mutex around a GPU, its commands, its queries, its two services and the ports of the kernel, the host and the programs it drives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wait"];
            let definition = "The step of an acquisition that waits for the users it names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Acquisition"];
            let definition = "The command that announces the demand of an eligible application for the GPU of a lock and returns once the application owns it, waiting for the GPU or ending its users as its mode says.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Acquisition", "Mode"];
            let definition = "How an acquisition takes the GPU, its deadline, whether it forces the users out, and whether it hands the GPU to vfio-pci.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Acquisition", "Step"];
            let definition = "What one round of an acquisition found, the ownership, the processes it must end first, or the users it waits for.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Ownership"];
            let definition = "The step of an acquisition whose application now owns the lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Displacement"];
            let definition = "The step of a forced acquisition whose processes must end before the application owns the lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Admission"];
            let definition = "The command that makes an application eligible for a lock, as its default owner where asked, and creates its cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dismissal"];
            let definition = "The command that makes an application ineligible for a lock again and removes its cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Enclosure"];
            let definition = "The command that puts a GPU under a lock, installs the device program where it is absent and locks the device files of the GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Release"];
            let definition = "The command that ends the ownership of the owner of a lock and gives the GPU back.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Refresh"];
            let definition = "The command that finds the device files of every lock again, after a GPU went to vfio-pci or came back from it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Removal"];
            let definition = "The command that takes a lock away with its records, its cgroups and its device files, and the device program with the last lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["User", "Query"];
            let definition = "The query of every user of the GPU of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status", "Query"];
            let definition = "The query of the state of every lock of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Guard", "Run"];
            let definition = "The command of the program without a privilege that acquires a lock for a program, runs it and releases the lock when it ends.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Administration"];
            let definition = "The application service of the helper that handles every command and query of a lock with the ports of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Guard"];
            let definition = "The application service of the program without a privilege that runs a program inside a lock through the helper.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Ports"];
            let definition = "The ports the lock administration drives, the device program, the device tree, the control group, the lock record, the process table and the pause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Guard", "Ports"];
            let definition = "The ports the lock guard drives, the host helper and the program runner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Setting"];
            let definition = "The interval of a check and the deadline between the request to end a process and its kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Record"];
            let definition = "The driven port that keeps the locks of the host under /run and holds them exclusively while a request changes them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Lease"];
            let definition = "The exclusive lease of the records of the locks, which ends when it is dropped.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Program"];
            let definition = "The driven port of the one device program at the root of the unified hierarchy and its maps of the device files and of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Tree"];
            let definition = "The driven port of a GPU in sysfs, its device files, its consoles and its binding to vfio-pci.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Control", "Group"];
            let definition = "The driven port of the cgroups of the eligible applications.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Task", "Table"];
            let definition = "The driven port of the processes of the host, the ones that hold a device file and their end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pause"];
            let definition = "The driven port that waits the interval of a check.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Helper"];
            let definition = "The driven port of the program without a privilege that runs baton_host through the elevation program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Runner"];
            let definition = "The driven port that runs the program of a guard run as a child and waits for its end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Deadline", "Expiry"];
            let definition = "The error of an acquisition whose deadline passed while the GPU stayed in use.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Absence"];
            let definition = "The error of a request for a lock that does not exist.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Owner", "Presence"];
            let definition = "The error of a removal of a lock that has an owner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["User", "Survival"];
            let definition = "The error of a forced acquisition whose user survived its kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "The error of a name that is no name of a lock or of an application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Item", "Type"];
            let definition = "The type of the items a list of texts is made from.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Items"];
            let definition = "The values a list of texts is made from.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record"];
            let definition = "The records of the locks of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock"];
            let definition = "The lock of a GPU, the module of its ports, its services and its setting.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device"];
            let definition = "The module of the ports of a GPU, its program and its tree.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tree"];
            let definition = "The device tree of a GPU in sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program"];
            let definition = "The device program at the root of the unified hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Guard"];
            let definition = "The lock guard and its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Administration"];
            let definition = "The lock administration and its steps.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Setting"];
            let definition = "The lock setting.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Step"];
            let definition = "The acquisition step.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mode"];
            let definition = "The acquisition mode.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Address", "Files"];
            let definition = "The device files of one GPU of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Interval"];
            let definition = "The interval between two checks of a wait.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Process", "Presence"];
            let definition = "To tell whether a process of the host still lives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Survival"];
            let definition = "To tell whether a process that was asked to end still lives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Demand", "Text"];
            let definition = "To write the owner and the demands of a lock a waiting acquisition names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Status", "Texts"];
            let definition = "To write the state of a lock as lines.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Texts"];
            let definition = "To write every item of a list as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Group", "Error"];
            let definition = "To turn an error of the control group into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Helper", "Error"];
            let definition = "To turn an error of the host helper into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Task", "Error"];
            let definition = "To turn an error of the process table into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Program", "Error"];
            let definition = "To turn an error of the device program into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Record", "Error"];
            let definition = "To turn an error of the lock record into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Runner", "Error"];
            let definition = "To turn an error of the program runner into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Tree", "Error"];
            let definition = "To turn an error of the device tree into an error of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Deadline"];
            let definition = "The time an acquisition waits at most.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Demand", "Texts"];
            let definition = "The demands of a lock as texts.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Persistence"];
            let definition = "Whether the persistence mode of one GPU was on before its passthrough.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Duration"];
            let definition = "The time a pause waits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["End", "Deadline"];
            let definition = "The time a process has between the request to end it and its kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit", "Code"];
            let definition = "The exit code of the program of a guard run.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Force", "Presence"];
            let definition = "Whether an acquisition ends the users of the GPU instead of waiting for them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Former", "Owner"];
            let definition = "The application that owned a lock before its release or its end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Identifier"];
            let definition = "The identifier of a cgroup, the inode of its directory, which the map of the permissions holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Guard", "Identifier"];
            let definition = "The identifier of the process of the guard, which waits for the demand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Application", "Name"];
            let definition = "The application whose processes a search of the users leaves out, where there is one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Deadline"];
            let definition = "The deadline of an acquisition, where it has one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Graphics", "Lock"];
            let definition = "The lock of a name among the records, where it exists.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Owner", "Name"];
            let definition = "The name of the owner of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Passthrough", "Text"];
            let definition = "Whether a lock is passed through, as on or off.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path", "Text"];
            let definition = "The path of a cgroup below the root of the unified hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Presence"];
            let definition = "Whether the dismissed application is the one the device program permits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permitted", "Application"];
            let definition = "The application the device program permits for a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Population", "Presence"];
            let definition = "Whether a cgroup holds a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Texts"];
            let definition = "The program of a guard run and its arguments.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Release", "Outcome"];
            let definition = "The outcome of the release of a guard run.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Run", "Outcome"];
            let definition = "The outcome of the program of a guard run.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status", "Texts"];
            let definition = "The lines of the state of the locks.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Survivor", "Users"];
            let definition = "The processes that survived their kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Turn", "Presence"];
            let definition = "Whether the demand of an acquisition is the first of the queue.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["User", "Text"];
            let definition = "The users or the demands an acquisition waits for, as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wait", "Duration"];
            let definition = "The time a wait has lasted.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Acquire", "Lock"];
            let definition = "To ask the helper for the acquisition of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Advance", "Acquisition"];
            let definition = "To run one round of an acquisition with the records held.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Bind", "Console", "User"];
            let definition = "To bind a console or a framebuffer of the firmware to the GPU again.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Bind", "Passthrough"];
            let definition = "To hand a GPU to vfio-pci and answer whether its persistence mode was on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Check", "Population"];
            let definition = "To tell whether a cgroup holds a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Create", "Group"];
            let definition = "To create a cgroup with its parents and answer its identifier.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Delete", "Lock"];
            let definition = "To delete the record of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "End", "Process", "Users"];
            let definition = "To end the processes that use a GPU, first with the request to end and after the end deadline with the kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Scan", "Process", "Users"];
            let definition = "To find every process that holds one of the device files.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Scan", "Users"];
            let definition = "To find every user of the GPU of a lock, leaving out the processes of an application where one is named.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Force", "End"];
            let definition = "To kill a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Lease", "Records"];
            let definition = "To lease the records of the locks exclusively.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Install", "Program"];
            let definition = "To install the device program at the root of the unified hierarchy where it is absent.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Load", "Lock"];
            let definition = "To load the record of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Load", "Locks"];
            let definition = "To load the record of every lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Insert", "Device", "Files"];
            let definition = "To insert device files into the map of the devices for a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Move", "Process"];
            let definition = "To move a process into a cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Permit", "Application"];
            let definition = "To permit the cgroup of an application in the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Permit", "Group"];
            let definition = "To permit a cgroup for a lock in the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Read", "Console", "Users"];
            let definition = "To find the consoles and the framebuffers of the firmware drawn on a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Read", "Device", "Files"];
            let definition = "To find the device files of a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Read", "Group", "Identifier"];
            let definition = "To read the identifier of a cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Read", "Status"];
            let definition = "To ask the helper for the state of every lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Release", "Lock"];
            let definition = "To ask the helper for the release of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Release", "Passthrough"];
            let definition = "To give a GPU back from vfio-pci to the driver of the host, with its persistence mode where it was on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Remove", "Group"];
            let definition = "To remove a cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Remove", "Program"];
            let definition = "To detach the device program from the root of the unified hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Request", "End"];
            let definition = "To ask a process to end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Restore", "Lock"];
            let definition = "To give the GPU of a lock back after its owner, its passthrough, its consoles and its permission.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Revoke", "Application"];
            let definition = "To take the permission of an application out of the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Revoke", "Group"];
            let definition = "To take the permission of a cgroup for a lock out of the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Spawn", "Program"];
            let definition = "To run a program as a child and answer its exit code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Settle", "Lock"];
            let definition = "To drop the demands whose process ended and give back the GPU of an owner whose cgroup is empty.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Store", "Lock"];
            let definition = "To store the record of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Unbind", "Console", "User"];
            let definition = "To unbind a console or a framebuffer of the firmware from the GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Delete", "Device", "Files"];
            let definition = "To delete device files from the map of the devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Update", "Device", "Files"];
            let definition = "To find the device files of the GPUs of a lock again, delete the files it no longer has from the map of the devices and insert the ones it has.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockApplicationContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockApplicationContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockApplicationContext::DIRECTORY_NAME;
        let term_texts = StackLockApplicationContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_application_context_is_a_term() {
        let context_type = StackLockApplicationContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
