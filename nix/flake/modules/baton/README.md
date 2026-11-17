# baton

The NixOS module `nixosModules.default`. A host enables it with `baton.enable` and names its locks in
`baton.locks`, each with the PCI address of its GPU, its eligible applications and an optional default
owner.

- A rule of udev runs the enclosure of a lock when a driver other than vfio-pci binds its GPU, at the
  start of the host before any service opens it and again after every passthrough. The enclosure
  encloses the GPU and admits its applications, or refreshes a lock that already stands.
- A second rule of udev runs `baton_host refresh` whenever a node with a device file appears below the
  slot of the GPU, so a node of a driver that probes late is locked before any program hears of it.
- The activation admits an application that a rebuild adds to a lock that already stands.
- `baton_host` runs wrapped with a fixed `PATH` of `con2fbmap` and the programs of the system, such as
  `nvidia-smi`, since sudo from a service or a rule of udev hands it no `PATH` that holds them.
- The members of `baton.group` run `baton_host acquire`, `release` and `status` through sudo without a
  password, and nothing else of the helper.
- `baton.hostProgram` is the path of the wrapped `baton_host`, the one path sudo allows, for a
  program of another project that acquires a lock for its own process, such as a hypervisor.
- `baton.program` is the path of `baton`, wrapped with the paths of sudo and of `baton_host`. A module
  of another project runs its program inside a lock with
  `${config.baton.program} run --lock <lock> --application <application> -- <program>` and adds its
  application to `baton.locks.<lock>.applications`.
