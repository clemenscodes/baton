# Purity

A command does not change an aggregate in place. `Command::change_aggregate` takes the aggregate and
returns a `Change`, the changed aggregate together with the outcome of the command. A
projection is converted into its next read model, `From<(Self, &Event)>`, from the read model
before it and an event. Apart from what std traits dictate, the `Formatter` of `Display` and,
in the tests, the `Context` of a poll and the aggregates the actor of their repository owns, nothing
in the crate takes `&mut`.
