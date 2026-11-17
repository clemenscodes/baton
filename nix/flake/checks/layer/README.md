# layer

The check `layer`, with the table of the tree of baton. It holds the crates to
the table of `docs/ARCHITECTURE.md`. A type names its layer in `type Layer = ...;`, read from what
`tool_vocabulary --tokens` prints, and that layer must be the one of its directory. In every family under
`crates/stack` and `crates/products` the domain lies under `core`, the application under `application`,
the infrastructure under `adapter` and the presentation under `presentation`. In the coverage and in the
language of the vocabulary the domain lies under `core`, the application under `application`, the
infrastructure under `adapter` and the presentation under `command_line`, and in any other tool the layer
is the one of its module `src/<layer>`. The crate `stack_domain_driven_design_core` declares the layers and
names none.

It also refuses a production dependency that points outward. A crate of a core reaches only the crates of
its own core and `stack_domain_driven_design_core`, and a core of a product reaches
the cores of the stack as well. The stack knows no product, the capture knows no display and the display knows
no capture, because the product composes the two. A presentation reaches no crate of a core. A tool reaches
only `stack_domain_driven_design_core`, the crates of the language of the vocabulary and, in the coverage,
the crates of the coverage.

It also holds every crate to the tag of its layer in its `moon.yml`. A crate carries `infrastructure` under
an `adapter`, `presentation` only as a program with a `src/main.rs`, because a library is never a
presentation, `domain` under a `core` and `application` under an `application`. The gates and
`stack_domain_driven_design_core` carry no tag of a layer. A crate the table does not cover is refused as
well.
