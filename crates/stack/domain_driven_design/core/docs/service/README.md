# The service

A service holds no aggregate of its own. `Service::try_dispatch_command` takes one `Dispatch`,
made by `Dispatch::from((identity, command))`, hands it to `try_change_aggregate` of its
repository and returns the outcome of the command. The state lives in the adapter of the
repository, and the adapter reads the aggregate, has the command compute the change and saves
the changed aggregate in one step, so two commands to one aggregate never overwrite each other's
change. A dispatch to an identity the repository keeps nothing under fails with `Error::Absence`,
and an error of the repository becomes `Error::Repository`, into which the error of the repository converts
through `From`. `SynchronousService` and `SynchronousRepository` are the same without futures.

A command, a query or an event that changes no aggregate of a repository goes to its handler
instead: `CommandHandler::dispatch_command`, `QueryHandler::resolve_query` and
`EventHandler::receive_event` return a future of their outcome, and `SynchronousCommandHandler`,
`SynchronousQueryHandler` and `SynchronousEventHandler` are the same without futures.

`LocalCommandHandler`, `LocalQueryHandler` and `LocalEventHandler` return futures that need not be `Send`: their work
stays on the thread that awaits it, as in a browser or over a cell that is not shared between
threads.
