# The ubiquitous language

| Word | What it means |
|---|---|
| Absence | The aggregate presence of an identity the repository keeps no aggregate under, the outcome presence of a dispatch to such an identity, and the error of a dispatch to such an identity. |
| Acceptance | The decision about a candidate that meets a specification. |
| Adapter | A type of the infrastructure layer that fills a port. |
| Add Aggregate | To keep an aggregate in a repository under its identity only where the repository keeps no aggregate under that identity yet, in one step no other call of the repository comes between, so that a stored aggregate is never replaced. |
| Addition | The response of a repository to an aggregate it is to add, an insertion or a collision. |
| Aggregate | A cluster of domain objects that changes as one and is reached through its root. |
| Aggregate Presence | The response of a repository to a read or a deletion of the aggregate of an identity, a presence or an absence, converted from the stored aggregate of that identity. |
| Aggregate Root | The entity through which an aggregate is reached, read, changed and saved as a whole. |
| Aggregates | The aggregates a repository reads for several identities, in their order. |
| Anti Corruption Layer | A translation at the boundary of a bounded context that keeps a foreign model out of its own model. |
| Append Event | To add an event at the end of an event store. |
| Application Layer | The layer of the application, whose services drive the domain through its ports and own the order of the steps. |
| Application Service | A service of the application layer, which drives the domain through its ports. |
| Blueprint | What a factory creates its product out of. |
| Bounded Context | The boundary within which one model and its language hold. |
| Bus | The event bus, and the module of it. |
| Candidate | What a specification decides the acceptance of. |
| Candidate Decision | The decision of a specification about a candidate, an acceptance or a rejection; two candidate decisions combine by `&` and by `\|`, and `!` turns one around. |
| Change | The changed aggregate a command yields together with the outcome of the command. |
| Change Aggregate | To change an aggregate by a command, which yields its change, and to change the aggregate of the identity of a dispatch in a repository by the command of the dispatch, read, changed and saved in one step no other call of the repository comes between, which responds with the outcome presence. |
| Collision | The addition that keeps nothing new, because the repository already keeps an aggregate under the identity, and that aggregate stays as it was. |
| Command | A request to change an aggregate, which computes the changed aggregate and an outcome. |
| Command Handler | A handler that dispatches the commands of one type, which it does not bind to the trait command, and yields a future of its outcome. |
| Commit Changes | To make every change of a work unit hold at once. |
| Compute Reaction | To compute the reaction of a policy to an event. |
| Conjunction | A specification that accepts what both of its specifications accept, which `&` yields. |
| Decide Acceptance | To decide the acceptance of a candidate by a specification. |
| Delete Aggregate | To remove the aggregate of an identity from a repository, which responds with the aggregate presence the identity had, a presence that holds the aggregate it removed, or an absence where it kept none. |
| Disjunction | A specification that accepts what either of its specifications accepts, which `\|` yields. |
| Dispatch | A command together with the identity of the aggregate a service dispatches it to and a repository changes by it, and the module of it. |
| Dispatch Command | To carry a command to what it changes and return its outcome, as a command handler does, and as a service does with a dispatch, by handing the dispatch to its repository, which changes the aggregate of its identity by its command in one step. |
| Domain | The part of the world a model describes, the layer that holds that model and nothing else, and the module of the domain event and the domain service. |
| Domain Driven Design | The design of software around the model of its domain, and this crate of its building blocks, which every crate names in the paths of its markers and its ports. |
| Domain Event | An event of the domain layer, a fact of the domain that happened. |
| Domain Layer | The layer of the domain, its model, the rules of its business and its ports. |
| Domain Service | An operation of the domain layer that belongs to no entity and to no value object. |
| Driven Port | A port the domain calls to reach the world, which only an adapter fills. |
| Driving Port | A port the presentation calls to drive the application. |
| Entity | A domain object its identity tells from every other, whatever the values of its attributes. |
| Error | An error: a dispatch returns it as an absence or as the error of its repository, and a repository, an event bus, an event store and a work unit name theirs in their associated type. |
| Event | A fact of the domain that happened and that other parts may react to, and the module of the event bus, the event handler and the event store. |
| Event Bus | The trait through which domain events are published to whoever listens. |
| Event Handler | A handler that receives one kind of domain event and yields a future of its outcome. |
| Event Store | The trait that appends domain events and loads every event it keeps, in the order they happened. |
| Events | The events an event store loads, in the order it keeps them. |
| Factory | A domain object that creates another out of a blueprint. |
| Format Result | What the formatting of an error into text returns. |
| Formatter | The formatter of the standard library that the display of an error writes its text into. |
| Handler | A type of the application layer that receives one kind of command, query or event, and the module of each such type. |
| Identifier | A value object that serves as the identity of an entity and hashes, so that a repository reaches an aggregate by it. |
| Identities | The identities whose aggregates a repository reads at once, in their order, which yield a reference to each identity. |
| Identity | What tells one entity from every other, whatever the values of its attributes. |
| Infrastructure Layer | The layer of the adapters that fill the ports. |
| Inner Decision | The decision the inner specification of a negation makes about a candidate. |
| Inner Specification | The specification whose decision a negation turns around. |
| Insertion | The addition that keeps the aggregate, because the repository kept no aggregate under its identity. |
| Layer | One of the four layers a type belongs to, the domain layer, the application layer, the infrastructure layer and the presentation layer, and the trait whose associated type names the layer of a type. |
| Left Decision | The decision the left specification of a conjunction or a disjunction makes about a candidate. |
| Left Specification | The first of the two specifications a conjunction or a disjunction is made of. |
| Load Events | To read every event an event store keeps. |
| Local Command Handler | A command handler whose future of its outcome stays on the thread that awaits it. |
| Local Event Handler | An event handler whose future of its outcome stays on the thread that awaits it. |
| Local Handler | The module of a handler whose future stays on the thread that awaits it and need not be sent to another, as in a browser or over a cell that is not shared between threads. |
| Local Query Handler | A query handler whose future of its output stays on the thread that awaits it. |
| Model | The state a component of the presentation keeps to itself, converted from its view. |
| Negation | A specification that accepts what its inner specification rejects and rejects what it accepts, which `!` yields. |
| Next Specification | The specification `&` or `\|` joins to a conjunction, a disjunction or a negation, the right specification of what it yields. |
| Outcome | What a command yields beside its changed aggregate, and what a command handler or an event handler yields to its caller. |
| Outcome Presence | The response of a repository to a dispatch it changes an aggregate by, a presence that holds the outcome of the command of the dispatch, or an absence where the repository keeps no aggregate under the identity of the dispatch. |
| Output | What a query handler resolves a query into, the associated type of the query. |
| Policy | A rule of the domain that computes a reaction to an event. |
| Port | A trait through which the domain reaches the world or the world drives the application. |
| Presence | The aggregate presence of an identity the repository keeps an aggregate under, which holds that aggregate, and the outcome presence of a dispatch to such an identity, which holds the outcome of its command. |
| Presentation | The layer a person or a program reaches, and a type of that layer that names the model it presents. |
| Presentation Layer | The layer a person or a program reaches. |
| Product | What a factory creates. |
| Projection | A read model whose next read model is converted from the read model before it and an event. |
| Publish Event | To hand an event to an event bus. |
| Query | A request for data that changes nothing. |
| Query Handler | A handler that resolves one kind of query into a future of its output. |
| Reaction | What a policy computes from an event. |
| Read Aggregate | To fetch the aggregate presence of an identity from a repository, a presence that holds its aggregate where the repository keeps one, and an absence where it keeps none. |
| Read Aggregates | To fetch the aggregates of the identities a caller hands a repository at once, in the order of the identities, leaving out every identity the repository keeps nothing under. |
| Read Model | A model shaped for reading, computed from the state or the events of the domain. |
| Receive Event | To react to an event and yield the outcome, as an event handler does. |
| Rejection | The decision about a candidate that fails a specification. |
| Repository | The trait through which aggregates are read, added, saved, changed by a command and deleted by their identity, and the error of a dispatch that fails in the repository. |
| Resolve Query | To turn a query into its output, as a query handler does. |
| Right Decision | The decision the right specification of a conjunction or a disjunction makes about a candidate, the decision on the right of `&` or `\|`. |
| Right Specification | The second of the two specifications a conjunction or a disjunction is made of. |
| Saga | A type of the application layer that coordinates a process across aggregates. |
| Save Aggregate | To keep an aggregate in a repository under its identity, in place of whatever aggregate the repository kept under it. |
| Seal | The trait no crate but this one implements, which closes the set of the four layers. |
| Sequence | The elements any collection is collected from, in their order, such as the aggregates, events or identities of a collection here, and the books, foreign languages, regions or test boundaries of the crates that build on it. |
| Service | A service of the application that dispatches commands to the aggregates of one repository. |
| Shared Kernel | The part of a model that more than one bounded context shares. |
| Specification | A rule of the domain that decides the acceptance of a candidate. |
| Stack Domain Driven Design Core | The crate stack_domain_driven_design_core, the building blocks of domain-driven design as traits. |
| Store | The event store, and the module of it. |
| Stored Aggregate | The aggregate a repository keeps under an identity, if it keeps one. |
| Synchronous Command Handler | A command handler whose dispatch returns its outcome when it is done, without a future. |
| Synchronous Event Handler | An event handler that yields its outcome when it is done, without a future. |
| Synchronous Handler | The module of a handler whose every call returns when it is done, without a future. |
| Synchronous Query Handler | A query handler that resolves a query into its output when it is done, without a future. |
| Synchronous Repository | A repository whose every call returns when it is done, without a future. |
| Synchronous Service | A service that dispatches through a synchronous repository, without a future. |
| Try Create Product | To create the product of a factory out of a blueprint, which may borrow what it reads, or to fail with the error of the factory where the blueprint breaks an invariant of the product. |
| Value Object | A domain object its values alone define, which is cloned and compared as a whole. |
| View | The published contract of a component of the presentation, which its callers build and pass, and from which the component converts its model. |
| Work Unit | A set of changes that is committed as one or not at all. |
