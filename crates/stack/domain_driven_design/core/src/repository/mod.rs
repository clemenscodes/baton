mod addition;
mod aggregate_presence;
mod aggregates;
mod identities;
mod outcome_presence;
mod synchronous_repository;

use crate::aggregate_root::AggregateRoot;
use crate::command::Command;
use crate::dispatch::Dispatch;
use std::future::Future;

pub use addition::Addition;
pub use aggregate_presence::AggregatePresence;
pub use aggregates::Aggregates;
pub use identities::Identities;
pub use outcome_presence::OutcomePresence;
pub use synchronous_repository::SynchronousRepository;

pub trait Repository<Aggregate: AggregateRoot> {
    type Error;

    fn try_read_aggregate(
        &self,
        identity: &Aggregate::Identity,
    ) -> impl Future<Output = Result<AggregatePresence<Aggregate>, Self::Error>> + Send;

    fn try_read_aggregates(
        &self,
        identities: &Identities<Aggregate::Identity>,
    ) -> impl Future<Output = Result<Aggregates<Aggregate>, Self::Error>> + Send;

    fn try_add_aggregate(
        &self,
        aggregate: &Aggregate,
    ) -> impl Future<Output = Result<Addition, Self::Error>> + Send;

    fn try_save_aggregate(
        &self,
        aggregate: &Aggregate,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn try_change_aggregate<CommandType>(
        &self,
        dispatch: Dispatch<Aggregate::Identity, CommandType>,
    ) -> impl Future<Output = Result<OutcomePresence<CommandType::Outcome>, Self::Error>> + Send
    where
        CommandType: Command<Aggregate> + Send + 'static,
        CommandType::Outcome: Send + 'static;

    fn try_delete_aggregate(
        &self,
        identity: &Aggregate::Identity,
    ) -> impl Future<Output = Result<AggregatePresence<Aggregate>, Self::Error>> + Send;
}

#[cfg(test)]
pub(super) mod tests {
    use super::Addition;
    use super::AggregatePresence;
    use super::Repository;

    pub(crate) mod fixture {
        use crate::adapter::Adapter;
        use crate::aggregate_root::AggregateRoot;
        use crate::command::Command;
        use crate::dispatch::Dispatch;
        use crate::entity::Entity;
        use crate::identifier::Identifier;
        use crate::layer::DomainLayer;
        use crate::layer::InfrastructureLayer;
        use crate::layer::Layer;
        use crate::repository::Addition;
        use crate::repository::AggregatePresence;
        use crate::repository::Aggregates;
        use crate::repository::Identities;
        use crate::repository::OutcomePresence;
        use crate::repository::Repository;
        use crate::repository::SynchronousRepository;
        use crate::value_object::ValueObject;
        use std::fmt;

        pub(crate) mod repository {
            use crate::adapter::Adapter;
            use crate::dispatch::Dispatch;
            use crate::entity::Entity;
            use crate::layer::InfrastructureLayer;
            use crate::layer::Layer;
            use crate::repository::Addition;
            use crate::repository::AggregatePresence;
            use crate::repository::Aggregates;
            use crate::repository::Identities;
            use crate::repository::OutcomePresence;
            use crate::repository::SynchronousRepository;
            use crate::repository::tests::fixture::Aggregate;
            use crate::repository::tests::fixture::Identity;
            use std::collections::BTreeMap;
            use std::fmt;
            use std::thread;
            use tokio::runtime::Builder;
            use tokio::sync::mpsc;
            use tokio::sync::mpsc::error::SendError;
            use tokio::sync::oneshot;
            use tokio::sync::oneshot::error::RecvError;

            type AggregateMap = BTreeMap<Identity, Aggregate>;

            #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
            pub(crate) enum Error {
                Sender,
                Receiver,
            }

            impl Layer for Error {
                type Layer = InfrastructureLayer;
            }

            impl fmt::Display for Error {
                fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    match self {
                        Self::Sender => {
                            write!(formatter, "the request cannot reach the repository actor")
                        }
                        Self::Receiver => {
                            write!(
                                formatter,
                                "the repository actor stopped before it responded"
                            )
                        }
                    }
                }
            }

            impl std::error::Error for Error {}

            impl From<SendError<Command>> for Error {
                fn from(_: SendError<Command>) -> Self {
                    Self::Sender
                }
            }

            impl From<RecvError> for Error {
                fn from(_: RecvError) -> Self {
                    Self::Receiver
                }
            }

            pub(crate) struct Command {
                change: Box<dyn FnOnce(&mut AggregateMap) + Send>,
            }

            impl Layer for Command {
                type Layer = InfrastructureLayer;
            }

            impl<ChangeType, Outcome> From<(ChangeType, oneshot::Sender<Outcome>)> for Command
            where
                ChangeType: FnOnce(&mut AggregateMap) -> Outcome + Send + 'static,
                Outcome: Send + 'static,
            {
                fn from(
                    (aggregate_change, oneshot_sender): (ChangeType, oneshot::Sender<Outcome>),
                ) -> Self {
                    let command_change = move |aggregates: &mut AggregateMap| {
                        let outcome = aggregate_change(aggregates);
                        match oneshot_sender.send(outcome) {
                            Ok(()) => {}
                            Err(..) => {
                                eprintln!("the requester stopped waiting for its response");
                            }
                        }
                    };
                    let change = Box::new(command_change);
                    Self { change }
                }
            }

            #[derive(Clone, Debug)]
            pub(crate) struct Repository {
                sender: mpsc::Sender<Command>,
            }

            impl Repository {
                async fn try_dispatch_command<ChangeType, Outcome>(
                    &self,
                    change: ChangeType,
                ) -> Result<Outcome, Error>
                where
                    ChangeType: FnOnce(&mut AggregateMap) -> Outcome + Send + 'static,
                    Outcome: Send + 'static,
                {
                    let Self { sender } = self;
                    let (oneshot_sender, oneshot_receiver) = oneshot::channel();
                    let command = Command::from((change, oneshot_sender));
                    sender.send(command).await?;
                    let outcome = oneshot_receiver.await?;
                    Ok(outcome)
                }

                fn try_block_on_command<ChangeType, Outcome>(
                    &self,
                    change: ChangeType,
                ) -> Result<Outcome, Error>
                where
                    ChangeType: FnOnce(&mut AggregateMap) -> Outcome + Send + 'static,
                    Outcome: Send + 'static,
                {
                    let Self { sender } = self;
                    let (oneshot_sender, oneshot_receiver) = oneshot::channel();
                    let command = Command::from((change, oneshot_sender));
                    sender.blocking_send(command)?;
                    let outcome = oneshot_receiver.blocking_recv()?;
                    Ok(outcome)
                }
            }

            impl FromIterator<Aggregate> for Repository {
                fn from_iter<AggregatesType: IntoIterator<Item = Aggregate>>(
                    aggregate_sequence: AggregatesType,
                ) -> Self {
                    let mut aggregates = AggregateMap::new();
                    for aggregate in aggregate_sequence {
                        let identity = *aggregate.identity();
                        aggregates.insert(identity, aggregate);
                    }
                    let capacity: usize = 8;
                    let (sender, mut receiver) = mpsc::channel(capacity);
                    let future = async move {
                        while let Some(command) = receiver.recv().await {
                            let Command { change } = command;
                            change(&mut aggregates);
                        }
                    };
                    let runtime = Builder::new_current_thread().build().unwrap();
                    let runtime_thread = move || runtime.block_on(future);
                    thread::spawn(runtime_thread);
                    Self { sender }
                }
            }

            impl Default for Repository {
                fn default() -> Self {
                    let aggregate_sequence: [Aggregate; 0] = [];
                    Self::from_iter(aggregate_sequence)
                }
            }

            impl Layer for Repository {
                type Layer = InfrastructureLayer;
            }

            impl Adapter for Repository {}

            impl SynchronousRepository<Aggregate> for Repository {
                type Error = Error;

                fn try_read_aggregate(
                    &self,
                    &identity: &Identity,
                ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let stored_aggregate = aggregates.get(&identity).copied();
                        AggregatePresence::from(stored_aggregate)
                    };
                    self.try_block_on_command(aggregate_change)
                }

                fn try_read_aggregates(
                    &self,
                    identities: &Identities<Identity>,
                ) -> Result<Aggregates<Aggregate>, Self::Error> {
                    let aggregate_identities = identities.clone();
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let stored_aggregate = |identity: &Identity| {
                            aggregates.get(identity).copied()
                        };
                        aggregate_identities
                            .into_iter()
                            .filter_map(stored_aggregate)
                            .collect()
                    };
                    self.try_block_on_command(aggregate_change)
                }

                fn try_add_aggregate(
                    &self,
                    &aggregate: &Aggregate,
                ) -> Result<Addition, Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let identity = *aggregate.identity();
                        match aggregates.get(&identity) {
                            Some(..) => Addition::Collision,
                            None => {
                                aggregates.insert(identity, aggregate);
                                Addition::Insertion
                            }
                        }
                    };
                    self.try_block_on_command(aggregate_change)
                }

                fn try_save_aggregate(&self, &aggregate: &Aggregate) -> Result<(), Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let identity = *aggregate.identity();
                        aggregates.insert(identity, aggregate);
                    };
                    self.try_block_on_command(aggregate_change)
                }

                fn try_change_aggregate<CommandType>(
                    &self,
                    dispatch: Dispatch<Identity, CommandType>,
                ) -> Result<OutcomePresence<CommandType::Outcome>, Self::Error>
                where
                    CommandType: crate::command::Command<Aggregate> + Send + 'static,
                    CommandType::Outcome: Send + 'static,
                {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let identity = dispatch.identity();
                        match aggregates.get(&identity).copied() {
                            Some(aggregate) => {
                                let command = dispatch.command();
                                let change = command.change_aggregate(aggregate);
                                let &stored_aggregate = change.aggregate();
                                let outcome = change.outcome();
                                aggregates.insert(identity, stored_aggregate);
                                OutcomePresence::Presence { outcome }
                            }
                            None => OutcomePresence::Absence,
                        }
                    };
                    self.try_block_on_command(aggregate_change)
                }

                fn try_delete_aggregate(
                    &self,
                    &identity: &Identity,
                ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let stored_aggregate = aggregates.remove(&identity);
                        AggregatePresence::from(stored_aggregate)
                    };
                    self.try_block_on_command(aggregate_change)
                }
            }

            impl crate::repository::Repository<Aggregate> for Repository {
                type Error = Error;

                async fn try_read_aggregate(
                    &self,
                    &identity: &Identity,
                ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let stored_aggregate = aggregates.get(&identity).copied();
                        AggregatePresence::from(stored_aggregate)
                    };
                    self.try_dispatch_command(aggregate_change).await
                }

                async fn try_read_aggregates(
                    &self,
                    identities: &Identities<Identity>,
                ) -> Result<Aggregates<Aggregate>, Self::Error> {
                    let aggregate_identities = identities.clone();
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let stored_aggregate = |identity: &Identity| {
                            aggregates.get(identity).copied()
                        };
                        aggregate_identities
                            .into_iter()
                            .filter_map(stored_aggregate)
                            .collect()
                    };
                    self.try_dispatch_command(aggregate_change).await
                }

                async fn try_add_aggregate(
                    &self,
                    &aggregate: &Aggregate,
                ) -> Result<Addition, Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let identity = *aggregate.identity();
                        match aggregates.get(&identity) {
                            Some(..) => Addition::Collision,
                            None => {
                                aggregates.insert(identity, aggregate);
                                Addition::Insertion
                            }
                        }
                    };
                    self.try_dispatch_command(aggregate_change).await
                }

                async fn try_save_aggregate(
                    &self,
                    &aggregate: &Aggregate,
                ) -> Result<(), Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let identity = *aggregate.identity();
                        aggregates.insert(identity, aggregate);
                    };
                    self.try_dispatch_command(aggregate_change).await
                }

                async fn try_change_aggregate<CommandType>(
                    &self,
                    dispatch: Dispatch<Identity, CommandType>,
                ) -> Result<OutcomePresence<CommandType::Outcome>, Self::Error>
                where
                    CommandType: crate::command::Command<Aggregate> + Send + 'static,
                    CommandType::Outcome: Send + 'static,
                {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let identity = dispatch.identity();
                        match aggregates.get(&identity).copied() {
                            Some(aggregate) => {
                                let command = dispatch.command();
                                let change = command.change_aggregate(aggregate);
                                let &stored_aggregate = change.aggregate();
                                let outcome = change.outcome();
                                aggregates.insert(identity, stored_aggregate);
                                OutcomePresence::Presence { outcome }
                            }
                            None => OutcomePresence::Absence,
                        }
                    };
                    self.try_dispatch_command(aggregate_change).await
                }

                async fn try_delete_aggregate(
                    &self,
                    &identity: &Identity,
                ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                    let aggregate_change = move |aggregates: &mut AggregateMap| {
                        let stored_aggregate = aggregates.remove(&identity);
                        AggregatePresence::from(stored_aggregate)
                    };
                    self.try_dispatch_command(aggregate_change).await
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Identity {
            identifier: u32,
        }

        impl From<u32> for Identity {
            fn from(identifier: u32) -> Self {
                Self { identifier }
            }
        }

        impl Layer for Identity {
            type Layer = DomainLayer;
        }

        impl ValueObject for Identity {}

        impl Identifier for Identity {}

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Aggregate {
            identity: Identity,
            value_object: u32,
        }

        impl From<Identity> for Aggregate {
            fn from(identity: Identity) -> Self {
                let value_object: u32 = 0;
                Self {
                    identity,
                    value_object,
                }
            }
        }

        impl From<(Identity, u32)> for Aggregate {
            fn from((identity, value_object): (Identity, u32)) -> Self {
                Self {
                    identity,
                    value_object,
                }
            }
        }

        impl Aggregate {
            pub(crate) fn value_object(&self) -> u32 {
                self.value_object
            }
        }

        impl Layer for Aggregate {
            type Layer = DomainLayer;
        }

        impl Entity for Aggregate {
            type Identity = Identity;

            fn identity(&self) -> &Self::Identity {
                &self.identity
            }
        }

        impl AggregateRoot for Aggregate {}

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Error;

        impl Layer for Error {
            type Layer = InfrastructureLayer;
        }

        impl fmt::Display for Error {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(formatter, "the repository rejects every request")
            }
        }

        impl std::error::Error for Error {}

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct ErrorRepository;

        impl Layer for ErrorRepository {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for ErrorRepository {}

        impl SynchronousRepository<Aggregate> for ErrorRepository {
            type Error = Error;

            fn try_read_aggregate(
                &self,
                _: &Identity,
            ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                let error = Error;
                Err(error)
            }

            fn try_read_aggregates(
                &self,
                _: &Identities<Identity>,
            ) -> Result<Aggregates<Aggregate>, Self::Error> {
                let error = Error;
                Err(error)
            }

            fn try_add_aggregate(&self, _: &Aggregate) -> Result<Addition, Self::Error> {
                let error = Error;
                Err(error)
            }

            fn try_save_aggregate(&self, _: &Aggregate) -> Result<(), Self::Error> {
                let error = Error;
                Err(error)
            }

            fn try_change_aggregate<CommandType>(
                &self,
                _: Dispatch<Identity, CommandType>,
            ) -> Result<OutcomePresence<CommandType::Outcome>, Self::Error>
            where
                CommandType: Command<Aggregate> + Send + 'static,
                CommandType::Outcome: Send + 'static,
            {
                let error = Error;
                Err(error)
            }

            fn try_delete_aggregate(
                &self,
                _: &Identity,
            ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                let error = Error;
                Err(error)
            }
        }

        impl Repository<Aggregate> for ErrorRepository {
            type Error = Error;

            async fn try_read_aggregate(
                &self,
                _: &Identity,
            ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                let error = Error;
                Err(error)
            }

            async fn try_read_aggregates(
                &self,
                _: &Identities<Identity>,
            ) -> Result<Aggregates<Aggregate>, Self::Error> {
                let error = Error;
                Err(error)
            }

            async fn try_add_aggregate(&self, _: &Aggregate) -> Result<Addition, Self::Error> {
                let error = Error;
                Err(error)
            }

            async fn try_save_aggregate(&self, _: &Aggregate) -> Result<(), Self::Error> {
                let error = Error;
                Err(error)
            }

            async fn try_change_aggregate<CommandType>(
                &self,
                _: Dispatch<Identity, CommandType>,
            ) -> Result<OutcomePresence<CommandType::Outcome>, Self::Error>
            where
                CommandType: Command<Aggregate> + Send + 'static,
                CommandType::Outcome: Send + 'static,
            {
                let error = Error;
                Err(error)
            }

            async fn try_delete_aggregate(
                &self,
                _: &Identity,
            ) -> Result<AggregatePresence<Aggregate>, Self::Error> {
                let error = Error;
                Err(error)
            }
        }
    }

    pub(crate) mod helper {
        use std::future::Future;
        use std::iter;
        use std::pin::pin;
        use std::task::Context;
        use std::task::Poll;
        use std::task::Waker;

        pub(crate) fn future_output<FutureType: Future>(future: FutureType) -> FutureType::Output {
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            let mut pin = pin!(future);
            loop {
                if let Poll::Ready(output) = pin.as_mut().poll(&mut context) {
                    return output;
                }
            }
        }

        pub(crate) fn future_outputs<FutureType: Future, const COUNT: usize>(
            futures: [FutureType; COUNT],
        ) -> [FutureType::Output; COUNT] {
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            let mut pins = futures.map(Box::pin);
            let mut outputs: [Option<FutureType::Output>; COUNT] = [const { None }; COUNT];
            while outputs.iter().any(Option::is_none) {
                for (pin, output) in iter::zip(&mut pins, &mut outputs) {
                    if output.is_none() {
                        let poll = pin.as_mut().poll(&mut context);
                        if let Poll::Ready(ready_output) = poll {
                            *output = Some(ready_output);
                        }
                    }
                }
            }
            outputs.map(Option::unwrap)
        }
    }

    #[test]
    fn an_addition_under_a_free_identity_keeps_the_aggregate_and_is_an_insertion() {
        let identifier: u32 = 1;
        let identity = fixture::Identity::from(identifier);
        let value_object: u32 = 2;
        let aggregate = fixture::Aggregate::from((identity, value_object));
        let repository = fixture::repository::Repository::default();
        let addition_future = repository.try_add_aggregate(&aggregate);

        let addition = helper::future_output(addition_future);

        let aggregate_future = repository.try_read_aggregate(&identity);
        let aggregate_presence = helper::future_output(aggregate_future);
        let insertion = Addition::Insertion;
        let presence = AggregatePresence::Presence { aggregate };
        let addition_expectation = Ok(insertion);
        assert_eq!(addition, addition_expectation);
        let aggregate_presence_expectation = Ok(presence);
        assert_eq!(aggregate_presence, aggregate_presence_expectation);
    }

    #[test]
    fn an_addition_under_an_occupied_identity_keeps_the_stored_aggregate_and_is_a_collision() {
        let identifier: u32 = 1;
        let identity = fixture::Identity::from(identifier);
        let stored_aggregate = fixture::Aggregate::from(identity);
        let value_object: u32 = 7;
        let aggregate = fixture::Aggregate::from((identity, value_object));
        let aggregate_sequence = [stored_aggregate];
        let repository = fixture::repository::Repository::from_iter(aggregate_sequence);
        let addition_future = repository.try_add_aggregate(&aggregate);

        let addition = helper::future_output(addition_future);

        let aggregate_future = repository.try_read_aggregate(&identity);
        let aggregate_presence = helper::future_output(aggregate_future);
        let collision = Addition::Collision;
        let presence = AggregatePresence::Presence {
            aggregate: stored_aggregate,
        };
        let addition_expectation = Ok(collision);
        assert_eq!(addition, addition_expectation);
        let aggregate_presence_expectation = Ok(presence);
        assert_eq!(aggregate_presence, aggregate_presence_expectation);
    }
}
