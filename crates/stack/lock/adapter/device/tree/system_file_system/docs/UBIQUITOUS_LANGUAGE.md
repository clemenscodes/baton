# The ubiquitous language

| Word | What it means |
|---|---|
| Address Text | The PCI address of a function as a text. |
| Bind Absence | The text 0 that unbinds a console. |
| Bind File | The name bind of the file that binds a console or a device. |
| Bind Path | The file bind of a console, a driver or vfio-pci. |
| Bind Presence | The text 1 of a bound console and of a boot device. |
| Bind Text | The text of the file bind of a virtual console. |
| Binding Deadline | The time a write to sysfs and a change of a driver have at most. |
| Boot Name | The name boot_vga of the file that tells whether a GPU is the one the firmware drew on. |
| Boot Path | The file boot_vga of a function. |
| Boot Presence | Whether a function of a GPU is the one the firmware drew on. |
| Bridge Class | The class 0x0604 of a PCI bridge, which is never passed through. |
| Class Text | The class of a PCI function. |
| Command Text | The command of a program as a text in an error. |
| Completion Failure | The error of a program that ended without success. |
| Compose Console Bind | To compose the file bind of a virtual console. |
| Compose Device File | To turn the identity of a device file into a character device file. |
| Compose Device Path | To compose the directory of a PCI function in sysfs. |
| Compose Group File | To compose the file of the group of vfio of a function. |
| Compose System Path | To compose a path below the root of the system. |
| Configuration | The tree configuration of the adapter. |
| Console Limit | The highest virtual console the console program is asked about, 63. |
| Console Path | The directory of one virtual console. |
| Console Program | The program that tells which framebuffer a virtual console is drawn on, con2fbmap unless an option names another. |
| Dev Name | The name dev of the file of sysfs that holds the major and the minor of a device file. |
| Dev Text | The text of a file dev, the major and the minor joined by a colon. |
| Device Absence | The error of a PCI device that does not exist. |
| Device Directory | The directory sys/bus/pci/devices of the PCI devices. |
| Device Path | The directory of a PCI function in sysfs. |
| Device Tree | The adapter of the port DeviceTree, which reads a GPU in sysfs, unbinds and binds its consoles, and hands it to vfio-pci and back. |
| Device Tree Port | The port DeviceTree as the adapter imports it. |
| Directory Path | The directory whose files dev are gathered. |
| Directory Presence | Whether an entry is a directory and no link. |
| Drain Outcome | The outcome of the drain of an NVIDIA GPU. |
| Driver Directory | The directory sys/bus/pci/drivers of the PCI drivers. |
| Driver Link | The link driver of a function to its driver. |
| Driver Path | The directory of a driver in sysfs. |
| Driver Target | The target of the link driver of a function. |
| Driver Text | The name of a driver of a framebuffer of the firmware. |
| Entries | The entries of a directory of sysfs. |
| Entry | An entry of a directory of sysfs. |
| Entry Path | The path of an entry of a directory of sysfs. |
| File Name | The name of an entry of a directory. |
| File Type | The type of an entry of a directory. |
| Firmware Drivers | The drivers of the framebuffers of the firmware, efi-framebuffer, simple-framebuffer and vesa-framebuffer. |
| Framebuffer Console | The text frame buffer in the name of the console of the framebuffers, fbcon. |
| Framebuffer Prefix | The prefix fb of the name of a framebuffer. |
| Framebuffer Text | The number of a framebuffer as a text. |
| Framebuffer Texts | The numbers of the framebuffers of a GPU. |
| Function Address | The PCI address of an entry of the directory of the devices. |
| Function Path | The directory of a function in sysfs. |
| Function Text | The PCI address of a function as sysfs names it. |
| Function Texts | The functions of a slot or of a group. |
| Graphics Name | The name graphics of the directory of the framebuffers of a device. |
| Graphics Path | The directory of the framebuffers of a function. |
| Group Absence | The error of a GPU without an IOMMU group, so the IOMMU is off. |
| Group Bridge | The error of an IOMMU group that holds a bridge. |
| Group Directory | The directory dev/vfio of the files of the groups of vfio. |
| Group Link | The link iommu_group of a function to its group. |
| Group Member | The error of an IOMMU group that holds a function of another card. |
| Group Name | The number of the IOMMU group of a function. |
| Group Path | The file of the group of vfio of a function. |
| Group Target | The target of the link iommu_group of a function. |
| Index Text | The index of a virtual console as a text. |
| Information Name | The name information of the file of a GPU of the NVIDIA driver. |
| Information Path | The file of the information of a GPU of the NVIDIA driver. |
| Information Text | The text of the information of a GPU of the NVIDIA driver. |
| Line | A line of the information of a GPU of the NVIDIA driver. |
| Mapping Presence | Whether a virtual console is drawn on a framebuffer of the GPU. |
| Mapping Text | The answer of the console program for one virtual console. |
| Maybe Console Index | The index of a virtual console, where its name holds one. |
| Maybe Device File | The device file of a file dev, where it holds a major and a minor. |
| Maybe Driver Name | The name of the driver a function is bound to, where it is bound. |
| Maybe File Name | The last part of a path, where it has one. |
| Maybe Framebuffer Text | The number of the framebuffer a console is drawn on, where the console program names one. |
| Member Presence | Whether a function of a group belongs to the slot of the GPU. |
| Metadata | The metadata of the file of a group of vfio, whose identity is its major and its minor. |
| Minor Label | The label Device Minor of the minor in the information of a GPU of the NVIDIA driver. |
| Name File | The name name of the file of a virtual console. |
| Nvidia Directory | The directory proc/driver/nvidia/gpus of the information of the NVIDIA driver. |
| Nvidia Driver | The driver nvidia of an NVIDIA GPU. |
| Nvidia Files | The file of the NVIDIA driver of a GPU. |
| Nvidia Major | The major 195 of the files of the NVIDIA driver. |
| Nvidia Presence | Whether a GPU is bound to the NVIDIA driver. |
| Output Failure | The error of a write to sysfs that failed. |
| Override File | The name driver_override of the file that names the one driver a device may bind. |
| Override Path | The file driver_override of a function. |
| Path | The path of a file of sysfs. |
| Persistence Answer | The answer Enabled of nvidia-smi for a GPU in the persistence mode. |
| Persistence Program | The program of the persistence mode and the drain of an NVIDIA GPU, nvidia-smi unless an option names another. |
| Platform Directory | The directory sys/bus/platform/drivers of the drivers of the framebuffers of the firmware. |
| Probe File | The file sys/bus/pci/drivers_probe that lets the kernel find a driver for a device again. |
| Probe Path | The file drivers_probe of the PCI bus. |
| Read Names | To read the names in a directory, or nothing where it cannot be read. |
| Read Text | To read a file without its surrounding white space, or nothing where it cannot be read. |
| Receiver | The end of the channel through which the outcome of a write arrives within the deadline. |
| Relative Text | A path below the root of the system. |
| Restore Nvidia | To end the drain of an NVIDIA GPU and turn its persistence mode on again after a failed passthrough. |
| Scan Device Files | To gather the major and the minor of every file dev below a directory, without following a link. |
| Sender | The end of the channel through which a thread of a write sends its outcome. |
| Spawner | The spawner of stack_process_core_invocation that starts the console program and the persistence program. |
| Spawner Type | The type of the spawner a device tree holds. |
| Stack Lock Adapter Device Tree System File System | The crate stack_lock_adapter_device_tree_system_file_system, the adapter of the port DeviceTree of stack_lock_application over sysfs, the information of the NVIDIA driver, con2fbmap and nvidia-smi. |
| Start Failure | The error of a program that could not be started. |
| Start Instant | The moment a wait for the driver of a function began. |
| Status | The exit status of a program as the operating system reports it. |
| System Directory | The root below which sys, proc and dev are read, / unless an option names another. |
| Tree Configuration | The root of the system, the console program, the persistence program, the deadline of a binding and the interval of a check. |
| Try Check Console Map | To ask the console program whether a virtual console is drawn on one of the framebuffers of a GPU. |
| Try Move Function | To hand a function to vfio-pci, unbinding its driver, naming vfio-pci in its override and binding it within the deadline. |
| Try Read Group Functions | To find every function of the IOMMU group of a GPU, refusing a group that is absent, holds a bridge or a function of another card. |
| Try Read Slot Functions | To find every function of the slot of a GPU, refusing a device that does not exist. |
| Try Return Function | To give a function back from vfio-pci to the driver of the host. |
| Try Spawn Persistence | To run the persistence program with its arguments. |
| Try Spawn Program | To run a program with its arguments and answer its output. |
| Try Write Within Deadline | To write a file of sysfs in a thread of its own and give up at the deadline of the binding, since a write may block in the kernel. |
| Unbind File | The name unbind of the file that unbinds a device from its driver. |
| Unbind Path | The file unbind of a driver. |
| Vfio Driver | The driver vfio-pci of a function that is passed through. |
| Vfio Presence | Whether every function of a group is bound to vfio-pci. |
| Vtconsole Directory | The directory sys/class/vtconsole of the consoles of the kernel. |
| Vtconsole Name | The name of a virtual console, such as vtcon1. |
| Vtconsole Path | The directory of the virtual consoles. |
| Vtconsole Prefix | The prefix vtcon of the name of a virtual console. |
| Write Outcome | The outcome of a write in its own thread. |
| Write Path | The file a thread of a write writes. |
| Write Text | The text a thread of a write writes. |
