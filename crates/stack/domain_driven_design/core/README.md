# stack_domain_driven_design_core

The building blocks of domain driven design as traits, so that "this is an aggregate root",
"this is a domain event" and "this type belongs to the infrastructure" are facts the compiler
checks rather than conventions a reader has to remember. The library depends on no other
crate, names no runtime and no framework, and is the shared kernel every bounded context speaks.
Its tests reach `tokio` for the actor of their repository: the handle starts a runtime of `tokio`
on a thread of its own when it is constructed, the runtime runs the actor as a task over the
receiving end of a bounded channel, the asynchronous repository awaits the channels and the
synchronous repository blocks on them without a runtime.

## The building blocks

Every building block is a module of the crate, and a caller names it by its path, such as
`layer::Layer` or `specification::Specification`.

| Concern | Paths |
|---|---|
| Building blocks | `value_object::ValueObject`, `entity::Entity`, `identifier::Identifier`, `aggregate_root::AggregateRoot`, `factory::Factory` |
| Domain logic | `domain::DomainService`, `specification::Specification` with its `CandidateDecision`, `Conjunction`, `Disjunction` and `Negation`, `policy::Policy` |
| Application | `command::Command` and the `change::Change` it computes, `command::CommandHandler`, `command::LocalCommandHandler` and `command::SynchronousCommandHandler`, `query::Query`, `query::QueryHandler`, `query::LocalQueryHandler` and `query::SynchronousQueryHandler`, `application_service::ApplicationService`, `service::Service`, `service::SynchronousService`, the `dispatch::Dispatch` they and the repositories take, and `service::Error`, the error of a dispatch |
| Events | `domain::DomainEvent`, `event::EventHandler`, `event::LocalEventHandler`, `event::SynchronousEventHandler`, `event::EventBus`, `event::store::EventStore` with its `Events`, `projection::Projection`, `read_model::ReadModel` |
| Persistence | `repository::Repository` with its `AggregatePresence`, its `Identities`, its `Aggregates`, its `Addition` and its `OutcomePresence`, `repository::SynchronousRepository`, `work_unit::WorkUnit` |
| Hexagonal | `port::Port`, `port::DrivingPort`, `port::DrivenPort`, `adapter::Adapter` |
| Layers | `layer::Layer`, `layer::DomainLayer`, `layer::ApplicationLayer`, `layer::InfrastructureLayer`, `layer::PresentationLayer` |
| Presentation | `presentation::Presentation`, `model::Model`, `view::View` |
| Strategic | `bounded_context::BoundedContext`, `shared_kernel::SharedKernel`, `anti_corruption_layer::AntiCorruptionLayer`, `saga::Saga` |

## The book

- [Kinds of enforcement](docs/layer/README.md)
- [Specifications](docs/specification/README.md)
- [Purity](docs/command/README.md) and [the service](docs/service/README.md)
- [Repositories](docs/repository/README.md)
- [Presentation](docs/presentation/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
