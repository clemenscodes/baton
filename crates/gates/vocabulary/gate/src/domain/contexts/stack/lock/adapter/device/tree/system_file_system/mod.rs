use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterDeviceTreeSystemFileSystemContext;

impl Layer for StackLockAdapterDeviceTreeSystemFileSystemContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterDeviceTreeSystemFileSystemContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/device/tree/system_file_system";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Device", "Tree", "System", "File", "System",
            ];
            let definition = "The crate stack_lock_adapter_device_tree_system_file_system, the adapter of the port DeviceTree of stack_lock_application over sysfs, the information of the NVIDIA driver, con2fbmap and nvidia-smi.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Tree"];
            let definition = "The adapter of the port DeviceTree, which reads a GPU in sysfs, unbinds and binds its consoles, and hands it to vfio-pci and back.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Tree", "Port"];
            let definition = "The port DeviceTree as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tree", "Configuration"];
            let definition = "The root of the system, the console program, the persistence program, the deadline of a binding and the interval of a check.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The tree configuration of the adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["System", "Directory"];
            let definition = "The root below which sys, proc and dev are read, / unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Console", "Program"];
            let definition = "The program that tells which framebuffer a virtual console is drawn on, con2fbmap unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Persistence", "Program"];
            let definition = "The program of the persistence mode and the drain of an NVIDIA GPU, nvidia-smi unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Binding", "Deadline"];
            let definition = "The time a write to sysfs and a change of a driver have at most.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The spawner of stack_process_core_invocation that starts the console program and the persistence program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner", "Type"];
            let definition = "The type of the spawner a device tree holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Directory"];
            let definition = "The directory sys/bus/pci/devices of the PCI devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driver", "Directory"];
            let definition = "The directory sys/bus/pci/drivers of the PCI drivers.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Probe", "File"];
            let definition = "The file sys/bus/pci/drivers_probe that lets the kernel find a driver for a device again.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Nvidia", "Directory"];
            let definition = "The directory proc/driver/nvidia/gpus of the information of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vtconsole", "Directory"];
            let definition = "The directory sys/class/vtconsole of the consoles of the kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Platform", "Directory"];
            let definition = "The directory sys/bus/platform/drivers of the drivers of the framebuffers of the firmware.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Directory"];
            let definition = "The directory dev/vfio of the files of the groups of vfio.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Information", "Name"];
            let definition = "The name information of the file of a GPU of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Name"];
            let definition = "The name graphics of the directory of the framebuffers of a device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Boot", "Name"];
            let definition = "The name boot_vga of the file that tells whether a GPU is the one the firmware drew on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dev", "Name"];
            let definition = "The name dev of the file of sysfs that holds the major and the minor of a device file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name", "File"];
            let definition = "The name name of the file of a virtual console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bind", "File"];
            let definition = "The name bind of the file that binds a console or a device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unbind", "File"];
            let definition = "The name unbind of the file that unbinds a device from its driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Override", "File"];
            let definition = "The name driver_override of the file that names the one driver a device may bind.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Minor", "Label"];
            let definition = "The label Device Minor of the minor in the information of a GPU of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Framebuffer", "Prefix"];
            let definition = "The prefix fb of the name of a framebuffer.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vtconsole", "Prefix"];
            let definition = "The prefix vtcon of the name of a virtual console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Framebuffer", "Console"];
            let definition = "The text frame buffer in the name of the console of the framebuffers, fbcon.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bind", "Presence"];
            let definition = "The text 1 of a bound console and of a boot device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bind", "Absence"];
            let definition = "The text 0 that unbinds a console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Persistence", "Answer"];
            let definition = "The answer Enabled of nvidia-smi for a GPU in the persistence mode.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vfio", "Driver"];
            let definition = "The driver vfio-pci of a function that is passed through.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vfio", "Presence"];
            let definition = "Whether every function of a group is bound to vfio-pci.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Nvidia", "Driver"];
            let definition = "The driver nvidia of an NVIDIA GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Nvidia", "Presence"];
            let definition = "Whether a GPU is bound to the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Nvidia", "Major"];
            let definition = "The major 195 of the files of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Nvidia", "Files"];
            let definition = "The file of the NVIDIA driver of a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bridge", "Class"];
            let definition = "The class 0x0604 of a PCI bridge, which is never passed through.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Console", "Limit"];
            let definition = "The highest virtual console the console program is asked about, 63.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Firmware", "Drivers"];
            let definition = "The drivers of the framebuffers of the firmware, efi-framebuffer, simple-framebuffer and vesa-framebuffer.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "System", "Path"];
            let definition = "To compose a path below the root of the system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Device", "Path"];
            let definition = "To compose the directory of a PCI function in sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Console", "Bind"];
            let definition = "To compose the file bind of a virtual console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Group", "File"];
            let definition = "To compose the file of the group of vfio of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Device", "File"];
            let definition = "To turn the identity of a device file into a character device file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Scan", "Device", "Files"];
            let definition = "To gather the major and the minor of every file dev below a directory, without following a link.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Read", "Slot", "Functions"];
            let definition = "To find every function of the slot of a GPU, refusing a device that does not exist.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Read", "Group", "Functions"];
            let definition = "To find every function of the IOMMU group of a GPU, refusing a group that is absent, holds a bridge or a function of another card.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Check", "Console", "Map"];
            let definition = "To ask the console program whether a virtual console is drawn on one of the framebuffers of a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Driver", "Name"];
            let definition = "The name of the driver a function is bound to, where it is bound.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "File", "Name"];
            let definition = "The last part of a path, where it has one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Move", "Function"];
            let definition = "To hand a function to vfio-pci, unbinding its driver, naming vfio-pci in its override and binding it within the deadline.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Return", "Function"];
            let definition = "To give a function back from vfio-pci to the driver of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Restore", "Nvidia"];
            let definition = "To end the drain of an NVIDIA GPU and turn its persistence mode on again after a failed passthrough.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Write", "Within", "Deadline"];
            let definition = "To write a file of sysfs in a thread of its own and give up at the deadline of the binding, since a write may block in the kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Spawn", "Persistence"];
            let definition = "To run the persistence program with its arguments.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Spawn", "Program"];
            let definition = "To run a program with its arguments and answer its output.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Names"];
            let definition = "To read the names in a directory, or nothing where it cannot be read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Text"];
            let definition = "To read a file without its surrounding white space, or nothing where it cannot be read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Absence"];
            let definition = "The error of a GPU without an IOMMU group, so the IOMMU is off.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Bridge"];
            let definition = "The error of an IOMMU group that holds a bridge.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Member"];
            let definition = "The error of an IOMMU group that holds a function of another card.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Absence"];
            let definition = "The error of a PCI device that does not exist.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output", "Failure"];
            let definition = "The error of a write to sysfs that failed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Failure"];
            let definition = "The error of a program that could not be started.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Completion", "Failure"];
            let definition = "The error of a program that ended without success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Text"];
            let definition = "The command of a program as a text in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status"];
            let definition = "The exit status of a program as the operating system reports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Address", "Text"];
            let definition = "The PCI address of a function as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function", "Text"];
            let definition = "The PCI address of a function as sysfs names it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function", "Texts"];
            let definition = "The functions of a slot or of a group.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function", "Path"];
            let definition = "The directory of a function in sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function", "Address"];
            let definition = "The PCI address of an entry of the directory of the devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Path"];
            let definition = "The directory of a PCI function in sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driver", "Path"];
            let definition = "The directory of a driver in sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driver", "Link"];
            let definition = "The link driver of a function to its driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driver", "Target"];
            let definition = "The target of the link driver of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driver", "Text"];
            let definition = "The name of a driver of a framebuffer of the firmware.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Link"];
            let definition = "The link iommu_group of a function to its group.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Target"];
            let definition = "The target of the link iommu_group of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Name"];
            let definition = "The number of the IOMMU group of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Path"];
            let definition = "The file of the group of vfio of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Information", "Path"];
            let definition = "The file of the information of a GPU of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Information", "Text"];
            let definition = "The text of the information of a GPU of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Path"];
            let definition = "The directory of the framebuffers of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Framebuffer", "Text"];
            let definition = "The number of a framebuffer as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Framebuffer", "Texts"];
            let definition = "The numbers of the framebuffers of a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Framebuffer", "Text"];
            let definition = "The number of the framebuffer a console is drawn on, where the console program names one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mapping", "Text"];
            let definition = "The answer of the console program for one virtual console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mapping", "Presence"];
            let definition = "Whether a virtual console is drawn on a framebuffer of the GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Index", "Text"];
            let definition = "The index of a virtual console as a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vtconsole", "Path"];
            let definition = "The directory of the virtual consoles.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vtconsole", "Name"];
            let definition = "The name of a virtual console, such as vtcon1.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Console", "Path"];
            let definition = "The directory of one virtual console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Console", "Index"];
            let definition = "The index of a virtual console, where its name holds one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bind", "Path"];
            let definition = "The file bind of a console, a driver or vfio-pci.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bind", "Text"];
            let definition = "The text of the file bind of a virtual console.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unbind", "Path"];
            let definition = "The file unbind of a driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Override", "Path"];
            let definition = "The file driver_override of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Probe", "Path"];
            let definition = "The file drivers_probe of the PCI bus.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Boot", "Path"];
            let definition = "The file boot_vga of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Boot", "Presence"];
            let definition = "Whether a function of a GPU is the one the firmware drew on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Class", "Text"];
            let definition = "The class of a PCI function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Member", "Presence"];
            let definition = "Whether a function of a group belongs to the slot of the GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory", "Path"];
            let definition = "The directory whose files dev are gathered.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory", "Presence"];
            let definition = "Whether an entry is a directory and no link.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry", "Path"];
            let definition = "The path of an entry of a directory of sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dev", "Text"];
            let definition = "The text of a file dev, the major and the minor joined by a colon.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Device", "File"];
            let definition = "The device file of a file dev, where it holds a major and a minor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Drain", "Outcome"];
            let definition = "The outcome of the drain of an NVIDIA GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Instant"];
            let definition = "The moment a wait for the driver of a function began.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Path"];
            let definition = "The file a thread of a write writes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Text"];
            let definition = "The text a thread of a write writes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Outcome"];
            let definition = "The outcome of a write in its own thread.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sender"];
            let definition = "The end of the channel through which a thread of a write sends its outcome.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Receiver"];
            let definition = "The end of the channel through which the outcome of a write arrives within the deadline.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Relative", "Text"];
            let definition = "A path below the root of the system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line"];
            let definition = "A line of the information of a GPU of the NVIDIA driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entries"];
            let definition = "The entries of a directory of sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry"];
            let definition = "An entry of a directory of sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Name"];
            let definition = "The name of an entry of a directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Type"];
            let definition = "The type of an entry of a directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Metadata"];
            let definition = "The metadata of the file of a group of vfio, whose identity is its major and its minor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a file of sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Identity", "Presence"];
            let definition = "To tell whether sys/dev holds the number of a device file, so a device of the system holds it now.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Character", "Identity", "Directory"];
            let definition = "The directory sys/dev/char, which holds a link for the number of every character device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Block", "Identity", "Directory"];
            let definition = "The directory sys/dev/block, which holds a link for the number of every block device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identity", "Directory"];
            let definition = "The directory of the numbers of the kind of a device file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Input", "Presence"];
            let definition = "Whether a directory of sysfs is a device of the class input, such as a jack of the outputs of a GPU, whose holders never use the GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Input", "Subsystem"];
            let definition = "The class input of the kernel, the end of the link subsystem of an input device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Subsystem", "Name"];
            let definition = "The name of the link subsystem of a directory of sysfs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Subsystem", "Path"];
            let definition = "The target of the link subsystem of a directory of sysfs.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterDeviceTreeSystemFileSystemContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockAdapterDeviceTreeSystemFileSystemContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterDeviceTreeSystemFileSystemContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterDeviceTreeSystemFileSystemContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_adapter_device_tree_system_file_system_context_is_a_term()
    {
        let context_type = StackLockAdapterDeviceTreeSystemFileSystemContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
