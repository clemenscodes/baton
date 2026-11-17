# The ubiquitous language

| Word | What it means |
|---|---|
| Check Membership | To tell whether a process lives in a cgroup or below it. |
| Command Text | The command of a process, as /proc names it in comm. |
| Console Index | The index of a virtual console of the kernel under /sys/class/vtconsole, such as 1 for vtcon1. |
| Console User | A virtual console of the kernel drawn on a framebuffer of a GPU. |
| Control Group Text | The path of the cgroup of a process below the root of the unified hierarchy, such as /baton/rtx3080/resolve. |
| Device Text | The name of the device of a framebuffer of the firmware, such as efi-framebuffer.0. |
| Driver Text | The name of the driver a framebuffer of the firmware is bound to. |
| Framebuffer User | A framebuffer of the firmware on a GPU, such as efi-framebuffer.0, bound to its driver. |
| Graphics User | Something that holds a GPU, a process with an open device file of it, a virtual console drawn on its framebuffer, or a framebuffer of the firmware on it. |
| Maybe Process Identifier | The identifier of the process of a graphics user, where it is a process. |
| Member Prefix | The path of a cgroup with a slash after it, which every descendant of it begins with. |
| Member Text | The path of the cgroup a process is checked against. |
| Process Identifier | The identifier of a process of the host. |
| Process User | A process that holds a device file of a GPU, with its identifier, its command and its cgroup. |
| Stack Lock Core Graphics User | The crate stack_lock_core_graphics_user, a user of a GPU, a process that holds one of its device files or a console of the kernel drawn on it. |
| Text | The text of a graphics user. |
