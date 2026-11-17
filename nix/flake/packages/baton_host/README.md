# baton_host

The package `baton_host`, the helper of the crate `products_baton_presentation_host` that runs as
root. It installs the device program, puts a GPU under a lock, admits and dismisses its eligible
applications, grants the lock to one demand after the other and answers the state of every lock. It is
built with crane from the sources the crate reaches and holds the binary `baton_host`. The host allows
the elevation program to run exactly this binary, so `baton` reaches it without a privilege of its own.
