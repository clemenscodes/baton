# Repositories

A repository is the illusion of a collection in memory of every aggregate of its type,
reached by the identity of the root. It deals in whole aggregates: `try_read_aggregate`
responds with the `AggregatePresence` of an identity, a `Presence` that holds the aggregate
reconstituted or an `Absence` where the repository keeps none, and `try_read_aggregates` returns
the `Aggregates` of the `Identities` it receives, `try_add_aggregate` keeps a new aggregate,
`try_save_aggregate` replaces the stored aggregate as a whole, `try_change_aggregate` changes
the aggregate of the identity of a `Dispatch` by its command, and `try_delete_aggregate` removes
it and responds with the `AggregatePresence` the identity had, the aggregate it removed or an
`Absence`. An `AggregatePresence` converts from the `Option` of a lookup through `From`. The storage
behind it is a file, a database or another machine, so every call can wait and every call can
fail, with an `Error` that belongs to the implementation. The futures are
`impl Future + Send`, so a call allocates nothing and an implementation can run on a runtime with
many threads.

`try_add_aggregate` keeps the aggregate only where the repository keeps no aggregate under its
identity yet, in a single step that no other call of the repository can come between, and never
replaces a stored aggregate. It responds with an `Addition`: an `Insertion` where it kept the
aggregate, and a `Collision` where the identity was taken, in which case the stored aggregate
stays as it was. So a caller that creates an aggregate fails on a second one with the same
identity, without needing a read before the write that another caller could get in between.

`try_change_aggregate` reads the aggregate of the identity of the dispatch, has its command
compute the change and saves the changed aggregate, in a single step that no other call of the
repository can come between. It responds with an `OutcomePresence`: a `Presence` that holds the
outcome of the command, or an `Absence` where the repository keeps no aggregate under the
identity, in which case it changes nothing. Lost updates are prevented by one actor per aggregate:
an adapter that owns its aggregates is an actor, every command to one of them is read, changed
and saved inside that actor, and so a second command to the same aggregate waits until the
first is saved. The command and its outcome cross to the actor, so both are `Send` and
`'static`.

`Aggregates` and the `Events` an event store loads are collections: they are collected from
any sequence through `FromIterator`, yield their elements in order through `IntoIterator`, and
are empty by `Default`. `Identities` is collected and empty in the same way, and a reference to
it yields a reference to each identity in order through `IntoIterator`. Every trait whose calls can fail names its error:
`Repository`, `SynchronousRepository`, `EventBus`, `EventStore` and `WorkUnit` in their
associated type `Error`, and `Service` and `SynchronousService` with `service::Error`, the
error of a dispatch. None of them is bound to `port::Port`.
