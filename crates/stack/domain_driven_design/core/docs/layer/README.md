# Kinds of enforcement

- **Structural** traits require real behavior and so enforce the architecture: `Repository`,
  `Command`, `Service` and `Specification`.
- **Markers of intent** enforce little or nothing and make the role of a type legible:
  `DomainService`, `DomainEvent`, `ReadModel` and the strategic traits.
- **The layer of a type** is the associated type of `Layer`. A type implements `Layer` once,
  so a second layer on the same type is a conflicting implementation that the compiler rejects,
  which makes the four layers mutually exclusive without a negative trait bound. The set of
  layers is sealed: the associated type admits only the four layers of this crate. The role
  traits pin a layer through `Layer`: `ValueObject`, `Entity`, `Identifier`, `AggregateRoot`,
  `Factory`, `DomainService`, `DomainEvent`, `Specification` and `Policy` the domain,
  `ApplicationService`, `Service`, `SynchronousService`, `Command`, `CommandHandler`,
  `SynchronousCommandHandler`, `Query`, `QueryHandler`, `SynchronousQueryHandler`, `EventHandler`,
  `SynchronousEventHandler` and `Saga` the application, `Adapter` the infrastructure, and
  `Presentation`, `Model` and `View` the presentation. So `AggregateRoot` belongs to the domain and
  `ApplicationService` to the application, and one type can never be both. `DrivenPort`
  requires `Adapter`, so every type that fills a driven port, an adapter or the fixed
  implementation a test fills it with, is an adapter of the infrastructure, and a type that
  fills a driven port without being one does not compile. `ReadModel`,
  `Projection`, `Port`, `DrivingPort`, `Repository`, `SynchronousRepository`, `EventBus`, `EventStore`,
  `WorkUnit` and the strategic traits pin no layer. The types of the crate state their layer as
  well: `CandidateDecision`, `Addition`, `AggregatePresence`, `OutcomePresence`, `Identities`, `Aggregates`, `Events`,
  `Conjunction`, `Disjunction` and `Negation` belong to the domain, and `Change`, `Dispatch` and
  `service::Error` to the application.

None of this enforces the rule of dependencies, that an inner layer never names an outer one.
Only the graph of the crates does that, by leaving the dependency out. The traits label the
seams, and the crates are the wall.
