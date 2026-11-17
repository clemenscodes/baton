# The ubiquitous language

| Word | What it means |
|---|---|
| Address Text | The PCI address of a device as a text in an error. |
| Application Absence | The error of an application that is not eligible for a lock. |
| Application Names | The eligible applications of a lock, every application that may use its GPU at all. |
| Application Path Text | The path of the cgroup of an eligible application below the root of the unified hierarchy, /baton, then the name of the lock and then the name of the application. |
| Application Text | The name of an application as a text in an error. |
| Assign Demand Owner | To make the application of a queued demand the owner of a lock and take the demand out of the queue, and to leave a demand that is not queued alone. |
| Check Eligibility | To tell whether an application is eligible for a lock. |
| Check Presence | To tell whether a process still lives. |
| Clear Owner | To end the ownership of an owner that ended without a release. |
| Clear Passthrough | To forget the passthrough of a lock and answer whether its persistence mode was on. |
| Console Users | The consoles and the framebuffers of the firmware an acquisition unbound from the GPU of a lock, which its release binds again. |
| Default Owner | The eligible application that holds a lock whenever no other owns it. |
| Default Owner Conflict | The error of a second default owner of a lock. |
| Default Presence | Whether an admitted application becomes the default owner of its lock. |
| Device Addresses | The PCI addresses of the GPUs a lock holds. |
| Device Files | The device files of the GPUs of a lock the device program locks. |
| Device Repetition | The error of a device a lock already holds. |
| Drop Demand | To take the demand of a process out of the queue of a lock. |
| Eligible Name | The name of an eligible application of a lock. |
| Graphics Demands | The queue of the demands of a lock, in the order they were made. |
| Graphics Lock | The lock of a GPU, the mutex the kernel enforces, with its name, its index in the maps of the device program, its devices, its device files, its eligible applications, its default owner, its owner, its queue of demands, the consoles it unbound and its passthrough. |
| Lock Text | The name of a lock as a text in an error. |
| Map Index | The number of a lock in the maps of the device program, which tells the locks of one host apart. |
| Maybe Default Owner | The eligible application that holds a lock whenever no other owns it, such as the desktop of a system with a single GPU, where there is one. |
| Maybe First Demand | The demand a lock serves next, where it has one. |
| Maybe Owner | The eligible application that owns a lock, where one does. |
| Maybe Permitted Application | The application the device program permits to open the GPU of a lock, its owner or else its default owner, where there is one. |
| Owner Conflict | The error of a dismissal of the owner of a lock. |
| Owner Mismatch | The error of a release by an application that does not own the lock. |
| Owner Text | The owner of a lock as a text in an error, or nobody. |
| Passthrough Presence | Whether the GPU of a lock is handed to vfio-pci for a guest. |
| Persistence Presence | Whether the persistence mode of an NVIDIA GPU was on before its passthrough, so its release turns it on again. |
| Previous Files | The device files a lock locked before a replacement. |
| Record Console Users | To remember the consoles and the framebuffers an acquisition unbound, once each. |
| Record Passthrough | To remember that the GPU of a lock is handed to vfio-pci, and whether its persistence mode was on. |
| Recorded Demand | A demand the queue of a lock holds. |
| Remove Console Users | To empty the record of the consoles and the framebuffers and answer them, which a release binds again. |
| Repetition Presence | Whether the queue already holds a demand of the same process. |
| Replace Device Files | To set the device files a lock locks and to answer the files it no longer locks. |
| Retain Demands | To keep in the queue only the demands whose process still lives. |
| Stack Lock Core Graphics Lock | The crate stack_lock_core_graphics_lock, the lock of a GPU, the aggregate that holds its GPUs, its eligible applications, its owner and its queue of demands. |
| Tree Name | The name of the tree of every cgroup of baton below the root of the unified hierarchy, baton. |
| Tree Path Text | The path of the tree of the cgroups of a lock below the root of the unified hierarchy, /baton and then the name of the lock. |
| Try Add Device | To add a GPU to a lock, refusing one it already holds. |
| Try Admit | To make an application eligible for a lock, as its default owner where asked, refusing a second default owner. |
| Try Check Application | To refuse an application that is not eligible for a lock. |
| Try Dismiss | To make an application ineligible again with its demands and its default, refusing the owner. |
| Try Record Demand | To put the demand of an eligible application into the queue of a lock, once for every process. |
| Try Recover Owner | To set the owner of a lock from its record, refusing an application that is not eligible. |
| Try Release | To end the ownership of the owner of a lock, refusing any other application. |
