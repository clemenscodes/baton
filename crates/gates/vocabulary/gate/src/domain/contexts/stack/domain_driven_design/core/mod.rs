use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackDomainDrivenDesignCoreContext;

impl Layer for StackDomainDrivenDesignCoreContext {
    type Layer = DomainLayer;
}

impl StackDomainDrivenDesignCoreContext {
    const DIRECTORY_NAME: &'static str = "stack/domain_driven_design/core";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Domain", "Driven", "Design", "Core"];
            let definition = "The crate stack_domain_driven_design_core, the building blocks of domain-driven design as traits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The part of the world a model describes, the layer that holds that model and nothing else, and the module of the domain event and the domain service.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain", "Driven", "Design"];
            let definition = "The design of software around the model of its domain, and this crate of its building blocks, which every crate names in the paths of its markers and its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain", "Layer"];
            let definition = "The layer of the domain, its model, the rules of its business and its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain", "Event"];
            let definition = "An event of the domain layer, a fact of the domain that happened.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain", "Service"];
            let definition = "An operation of the domain layer that belongs to no entity and to no value object.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Layer"];
            let definition = "The layer of the application, whose services drive the domain through its ports and own the order of the steps.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Service"];
            let definition = "A service of the application layer, which drives the domain through its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Infrastructure", "Layer"];
            let definition = "The layer of the adapters that fill the ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presentation", "Layer"];
            let definition = "The layer a person or a program reaches.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Layer"];
            let definition = "One of the four layers a type belongs to, the domain layer, the application layer, the infrastructure layer and the presentation layer, and the trait whose associated type names the layer of a type.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Seal"];
            let definition = "The trait no crate but this one implements, which closes the set of the four layers.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Port"];
            let definition = "A trait through which the domain reaches the world or the world drives the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driving", "Port"];
            let definition = "A port the presentation calls to drive the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driven", "Port"];
            let definition = "A port the domain calls to reach the world, which only an adapter fills.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Adapter"];
            let definition = "A type of the infrastructure layer that fills a port.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Anti", "Corruption", "Layer"];
            let definition = "A translation at the boundary of a bounded context that keeps a foreign model out of its own model.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value", "Object"];
            let definition = "A domain object its values alone define, which is cloned and compared as a whole.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entity"];
            let definition = "A domain object its identity tells from every other, whatever the values of its attributes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identity"];
            let definition = "What tells one entity from every other, whatever the values of its attributes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identifier"];
            let definition = "A value object that serves as the identity of an entity and hashes, so that a repository reaches an aggregate by it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Aggregate"];
            let definition = "A cluster of domain objects that changes as one and is reached through its root.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Aggregate", "Root"];
            let definition = "The entity through which an aggregate is reached, read, changed and saved as a whole.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Aggregates"];
            let definition = "The aggregates a repository reads for several identities, in their order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identities"];
            let definition = "The identities whose aggregates a repository reads at once, in their order, which yield a reference to each identity.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bounded", "Context"];
            let definition = "The boundary within which one model and its language hold.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Shared", "Kernel"];
            let definition = "The part of a model that more than one bounded context shares.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command"];
            let definition = "A request to change an aggregate, which computes the changed aggregate and an outcome.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Change"];
            let definition = "The changed aggregate a command yields together with the outcome of the command.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Change", "Aggregate"];
            let definition = "To change an aggregate by a command, which yields its change, and to change the aggregate of the identity of a dispatch in a repository by the command of the dispatch, read, changed and saved in one step no other call of the repository comes between, which responds with the outcome presence.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Outcome"];
            let definition = "What a command yields beside its changed aggregate, and what a command handler or an event handler yields to its caller.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Handler"];
            let definition = "A type of the application layer that receives one kind of command, query or event, and the module of each such type.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Handler"];
            let definition = "A handler that dispatches the commands of one type, which it does not bind to the trait command, and yields a future of its outcome.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Query"];
            let definition = "A request for data that changes nothing.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "What a query handler resolves a query into, the associated type of the query.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Query", "Handler"];
            let definition = "A handler that resolves one kind of query into a future of its output.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Resolve", "Query"];
            let definition = "To turn a query into its output, as a query handler does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Event"];
            let definition = "A fact of the domain that happened and that other parts may react to, and the module of the event bus, the event handler and the event store.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Events"];
            let definition = "The events an event store loads, in the order it keeps them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Event", "Handler"];
            let definition = "A handler that receives one kind of domain event and yields a future of its outcome.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Receive", "Event"];
            let definition = "To react to an event and yield the outcome, as an event handler does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Event", "Bus"];
            let definition = "The trait through which domain events are published to whoever listens.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bus"];
            let definition = "The event bus, and the module of it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Publish", "Event"];
            let definition = "To hand an event to an event bus.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Event", "Store"];
            let definition = "The trait that appends domain events and loads every event it keeps, in the order they happened.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Store"];
            let definition = "The event store, and the module of it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Append", "Event"];
            let definition = "To add an event at the end of an event store.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Load", "Events"];
            let definition = "To read every event an event store keeps.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sequence"];
            let definition = "The elements any collection is collected from, in their order, such as the aggregates, events or identities of a collection here, and the books, foreign languages, regions or test boundaries of the crates that build on it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Policy"];
            let definition = "A rule of the domain that computes a reaction to an event.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compute", "Reaction"];
            let definition = "To compute the reaction of a policy to an event.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reaction"];
            let definition = "What a policy computes from an event.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Projection"];
            let definition = "A read model whose next read model is converted from the read model before it and an event.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Model"];
            let definition = "A model shaped for reading, computed from the state or the events of the domain.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["View"];
            let definition = "The published contract of a component of the presentation, which its callers build and pass, and from which the component converts its model.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Model"];
            let definition = "The state a component of the presentation keeps to itself, converted from its view.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presentation"];
            let definition = "The layer a person or a program reaches, and a type of that layer that names the model it presents.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Factory"];
            let definition = "A domain object that creates another out of a blueprint.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Create", "Product"];
            let definition = "To create the product of a factory out of a blueprint, which may borrow what it reads, or to fail with the error of the factory where the blueprint breaks an invariant of the product.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Blueprint"];
            let definition = "What a factory creates its product out of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Product"];
            let definition = "What a factory creates.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Repository"];
            let definition = "The trait through which aggregates are read, added, saved, changed by a command and deleted by their identity, and the error of a dispatch that fails in the repository.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stored", "Aggregate"];
            let definition = "The aggregate a repository keeps under an identity, if it keeps one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Aggregate", "Presence"];
            let definition = "The response of a repository to a read or a deletion of the aggregate of an identity, a presence or an absence, converted from the stored aggregate of that identity.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presence"];
            let definition = "The aggregate presence of an identity the repository keeps an aggregate under, which holds that aggregate, and the outcome presence of a dispatch to such an identity, which holds the outcome of its command.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Aggregate"];
            let definition = "To fetch the aggregate presence of an identity from a repository, a presence that holds its aggregate where the repository keeps one, and an absence where it keeps none.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Aggregates"];
            let definition = "To fetch the aggregates of the identities a caller hands a repository at once, in the order of the identities, leaving out every identity the repository keeps nothing under.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Add", "Aggregate"];
            let definition = "To keep an aggregate in a repository under its identity only where the repository keeps no aggregate under that identity yet, in one step no other call of the repository comes between, so that a stored aggregate is never replaced.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Addition"];
            let definition = "The response of a repository to an aggregate it is to add, an insertion or a collision.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insertion"];
            let definition = "The addition that keeps the aggregate, because the repository kept no aggregate under its identity.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Collision"];
            let definition = "The addition that keeps nothing new, because the repository already keeps an aggregate under the identity, and that aggregate stays as it was.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Outcome", "Presence"];
            let definition = "The response of a repository to a dispatch it changes an aggregate by, a presence that holds the outcome of the command of the dispatch, or an absence where the repository keeps no aggregate under the identity of the dispatch.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Save", "Aggregate"];
            let definition = "To keep an aggregate in a repository under its identity, in place of whatever aggregate the repository kept under it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Delete", "Aggregate"];
            let definition = "To remove the aggregate of an identity from a repository, which responds with the aggregate presence the identity had, a presence that holds the aggregate it removed, or an absence where it kept none.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Synchronous", "Repository"];
            let definition = "A repository whose every call returns when it is done, without a future.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Local", "Handler"];
            let definition = "The module of a handler whose future stays on the thread that awaits it and need not be sent to another, as in a browser or over a cell that is not shared between threads.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Local", "Command", "Handler"];
            let definition = "A command handler whose future of its outcome stays on the thread that awaits it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Local", "Event", "Handler"];
            let definition = "An event handler whose future of its outcome stays on the thread that awaits it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Local", "Query", "Handler"];
            let definition = "A query handler whose future of its output stays on the thread that awaits it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Synchronous", "Handler"];
            let definition = "The module of a handler whose every call returns when it is done, without a future.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Synchronous", "Command", "Handler"];
            let definition = "A command handler whose dispatch returns its outcome when it is done, without a future.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Synchronous", "Query", "Handler"];
            let definition = "A query handler that resolves a query into its output when it is done, without a future.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Synchronous", "Event", "Handler"];
            let definition = "An event handler that yields its outcome when it is done, without a future.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Synchronous", "Service"];
            let definition = "A service that dispatches through a synchronous repository, without a future.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Service"];
            let definition = "A service of the application that dispatches commands to the aggregates of one repository.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dispatch"];
            let definition = "A command together with the identity of the aggregate a service dispatches it to and a repository changes by it, and the module of it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dispatch", "Command"];
            let definition = "To carry a command to what it changes and return its outcome, as a command handler does, and as a service does with a dispatch, by handing the dispatch to its repository, which changes the aggregate of its identity by its command in one step.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Formatter"];
            let definition = "The formatter of the standard library that the display of an error writes its text into.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error"];
            let definition = "An error: a dispatch returns it as an absence or as the error of its repository, and a repository, an event bus, an event store and a work unit name theirs in their associated type.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence"];
            let definition = "The aggregate presence of an identity the repository keeps no aggregate under, the outcome presence of a dispatch to such an identity, and the error of a dispatch to such an identity.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Work", "Unit"];
            let definition = "A set of changes that is committed as one or not at all.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Commit", "Changes"];
            let definition = "To make every change of a work unit hold at once.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Saga"];
            let definition = "A type of the application layer that coordinates a process across aggregates.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Specification"];
            let definition = "A rule of the domain that decides the acceptance of a candidate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Candidate"];
            let definition = "What a specification decides the acceptance of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Candidate", "Decision"];
            let definition = "The decision of a specification about a candidate, an acceptance or a rejection; two candidate decisions combine by `&` and by `|`, and `!` turns one around.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Acceptance"];
            let definition = "The decision about a candidate that meets a specification.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rejection"];
            let definition = "The decision about a candidate that fails a specification.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decide", "Acceptance"];
            let definition = "To decide the acceptance of a candidate by a specification.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Conjunction"];
            let definition = "A specification that accepts what both of its specifications accept, which `&` yields.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Disjunction"];
            let definition = "A specification that accepts what either of its specifications accepts, which `|` yields.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Negation"];
            let definition = "A specification that accepts what its inner specification rejects and rejects what it accepts, which `!` yields.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Left", "Specification"];
            let definition = "The first of the two specifications a conjunction or a disjunction is made of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Right", "Specification"];
            let definition = "The second of the two specifications a conjunction or a disjunction is made of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inner", "Specification"];
            let definition = "The specification whose decision a negation turns around.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Next", "Specification"];
            let definition = "The specification `&` or `|` joins to a conjunction, a disjunction or a negation, the right specification of what it yields.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Format", "Result"];
            let definition = "What the formatting of an error into text returns.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Left", "Decision"];
            let definition = "The decision the left specification of a conjunction or a disjunction makes about a candidate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Right", "Decision"];
            let definition = "The decision the right specification of a conjunction or a disjunction makes about a candidate, the decision on the right of `&` or `|`.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inner", "Decision"];
            let definition = "The decision the inner specification of a negation makes about a candidate.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackDomainDrivenDesignCoreContext> for Context {
    type Error = Error;

    fn try_from(_: StackDomainDrivenDesignCoreContext) -> Result<Self, Self::Error> {
        let directory_name = StackDomainDrivenDesignCoreContext::DIRECTORY_NAME;
        let term_texts = StackDomainDrivenDesignCoreContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_domain_driven_design_context_is_a_term() {
        let stack_domain_driven_design_context = StackDomainDrivenDesignCoreContext;

        let context = Context::try_from(stack_domain_driven_design_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
