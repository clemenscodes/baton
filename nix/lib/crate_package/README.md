# crate_package

The build of the program of one crate by crane with the profile `release` and without its
tests, from the source of the crate that `nix/lib/crate_source` cuts out of the tree. The
packages `vocabulary` and `coverage` of the flake are built by it.
