mod error;
mod synchronous_service;

use crate::aggregate_root::AggregateRoot;
use crate::application_service::ApplicationService;
use crate::command::Command;
use crate::dispatch::Dispatch;
use crate::entity::Entity;
use crate::repository::OutcomePresence;
use crate::repository::Repository;
use std::future::Future;

pub use error::Error;
pub use synchronous_service::SynchronousService;

type ServiceError<ServiceType, Aggregate> = Error<
    <Aggregate as Entity>::Identity,
    <<ServiceType as Service<Aggregate>>::Repository as Repository<Aggregate>>::Error,
>;

pub trait Service<Aggregate: AggregateRoot>: ApplicationService {
    type Repository: Repository<Aggregate>;

    fn repository(&self) -> &Self::Repository;

    fn try_dispatch_command<CommandType>(
        &self,
        dispatch: Dispatch<Aggregate::Identity, CommandType>,
    ) -> impl Future<Output = Result<CommandType::Outcome, ServiceError<Self, Aggregate>>> + Send
    where
        Self: Sync,
        Self::Repository: Sync,
        Aggregate::Identity: Send,
        CommandType: Command<Aggregate> + Send + 'static,
        CommandType::Outcome: Send + 'static,
    {
        async move {
            let repository = self.repository();
            let identity = dispatch.identity();
            let outcome_presence = repository.try_change_aggregate(dispatch).await?;
            match outcome_presence {
                OutcomePresence::Presence { outcome } => Ok(outcome),
                OutcomePresence::Absence => {
                    let absence = Error::Absence { identity };
                    Err(absence)
                }
            }
        }
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::repository;
    use crate::repository::AggregatePresence;
    use crate::repository::Aggregates;
    use crate::repository::Identities;

    pub(crate) mod fixture {
        use crate::application_service::ApplicationService;
        use crate::change::Change;
        use crate::entity::Entity;
        use crate::layer::ApplicationLayer;
        use crate::layer::Layer;
        use crate::repository::Repository;
        use crate::repository::SynchronousRepository;
        use crate::repository::tests::fixture::Aggregate;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Command;

        impl Layer for Command {
            type Layer = ApplicationLayer;
        }

        impl crate::command::Command<Aggregate> for Command {
            type Outcome = u32;

            fn change_aggregate(self, stored_aggregate: Aggregate) -> Change<Aggregate, u32> {
                let identity = *stored_aggregate.identity();
                let value_object = stored_aggregate.value_object() + 1;
                let aggregate = Aggregate::from((identity, value_object));
                Change::from((aggregate, value_object))
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct Service<RepositoryType> {
            repository: RepositoryType,
        }

        impl<RepositoryType> From<RepositoryType> for Service<RepositoryType> {
            fn from(repository: RepositoryType) -> Self {
                Self { repository }
            }
        }

        impl<RepositoryType> Layer for Service<RepositoryType> {
            type Layer = ApplicationLayer;
        }

        impl<RepositoryType> ApplicationService for Service<RepositoryType> {}

        impl<RepositoryType: Repository<Aggregate>> crate::service::Service<Aggregate>
            for Service<RepositoryType>
        {
            type Repository = RepositoryType;

            fn repository(&self) -> &Self::Repository {
                match self {
                    Self { repository } => repository,
                }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
        pub(crate) struct SynchronousService<RepositoryType> {
            repository: RepositoryType,
        }

        impl<RepositoryType> From<RepositoryType> for SynchronousService<RepositoryType> {
            fn from(repository: RepositoryType) -> Self {
                Self { repository }
            }
        }

        impl<RepositoryType> Layer for SynchronousService<RepositoryType> {
            type Layer = ApplicationLayer;
        }

        impl<RepositoryType> ApplicationService for SynchronousService<RepositoryType> {}

        impl<RepositoryType: SynchronousRepository<Aggregate>>
            crate::service::SynchronousService<Aggregate> for SynchronousService<RepositoryType>
        {
            type Repository = RepositoryType;

            fn repository(&self) -> &Self::Repository {
                match self {
                    Self { repository } => repository,
                }
            }
        }
    }

    #[test]
    fn a_dispatch_changes_the_stored_aggregate_and_returns_the_outcome() {
        let identifier: u32 = 1;
        let identity = repository::tests::fixture::Identity::from(identifier);
        let aggregate = repository::tests::fixture::Aggregate::from(identity);
        let aggregate_sequence = [aggregate];
        let repository = repository::tests::fixture::repository::Repository::from_iter(
            aggregate_sequence,
        );
        let service_repository = repository.clone();
        let service = fixture::Service::from(service_repository);
        let command = fixture::Command;
        let dispatch = Dispatch::from((identity, command));
        let dispatch_future = service.try_dispatch_command(dispatch);

        let outcome = repository::tests::helper::future_output(dispatch_future);

        let aggregate_future = repository.try_read_aggregate(&identity);
        let aggregate_presence = repository::tests::helper::future_output(aggregate_future);
        let value_object: u32 = 1;
        let stored_aggregate = repository::tests::fixture::Aggregate::from((
            identity,
            value_object,
        ));
        let presence = AggregatePresence::Presence {
            aggregate: stored_aggregate,
        };
        let outcome_expectation = Ok(value_object);
        assert_eq!(outcome, outcome_expectation);
        let aggregate_presence_expectation = Ok(presence);
        assert_eq!(aggregate_presence, aggregate_presence_expectation);
    }

    #[test]
    fn a_dispatch_to_an_identity_the_repository_does_not_keep_is_an_error() {
        let identifier: u32 = 9;
        let identity = repository::tests::fixture::Identity::from(identifier);
        let repository = repository::tests::fixture::repository::Repository::default();
        let service = fixture::Service::from(repository);
        let command = fixture::Command;
        let dispatch = Dispatch::from((identity, command));
        let dispatch_future = service.try_dispatch_command(dispatch);

        let outcome = repository::tests::helper::future_output(dispatch_future);

        let absence = Error::Absence { identity };
        let expectation = Err(absence);
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn a_dispatch_leaves_every_aggregate_of_another_identity_as_it_was() {
        let identifier_sequence: [u32; 2] = [1, 2];
        let identity_sequence = identifier_sequence.map(repository::tests::fixture::Identity::from);
        let aggregate_sequence = identity_sequence.map(repository::tests::fixture::Aggregate::from);
        let repository = repository::tests::fixture::repository::Repository::from_iter(
            aggregate_sequence,
        );
        let service_repository = repository.clone();
        let service = fixture::Service::from(service_repository);
        let command = fixture::Command;
        let [identity, ..] = identity_sequence;
        let dispatch = Dispatch::from((identity, command));
        let dispatch_future = service.try_dispatch_command(dispatch);

        let outcome = repository::tests::helper::future_output(dispatch_future);

        let identities = Identities::from_iter(identity_sequence);
        let aggregates_future = repository.try_read_aggregates(&identities);
        let aggregates = repository::tests::helper::future_output(aggregates_future);
        let value_object: u32 = 1;
        let stored_aggregate = repository::tests::fixture::Aggregate::from((
            identity,
            value_object,
        ));
        let [.., aggregate] = aggregate_sequence;
        let stored_aggregate_sequence = [stored_aggregate, aggregate];
        let expectation = Aggregates::from_iter(stored_aggregate_sequence);
        let outcome_expectation = Ok(value_object);
        assert_eq!(outcome, outcome_expectation);
        let aggregates_expectation = Ok(expectation);
        assert_eq!(aggregates, aggregates_expectation);
    }

    #[test]
    fn a_dispatch_the_repository_fails_on_is_an_error() {
        let identifier: u32 = 1;
        let identity = repository::tests::fixture::Identity::from(identifier);
        let error_repository = repository::tests::fixture::ErrorRepository;
        let service = fixture::Service::from(error_repository);
        let command = fixture::Command;
        let dispatch = Dispatch::from((identity, command));
        let dispatch_future = service.try_dispatch_command(dispatch);

        let outcome = repository::tests::helper::future_output(dispatch_future);

        let repository_error = repository::tests::fixture::Error;
        let error = Error::Repository {
            error: repository_error,
        };
        let expectation = Err(error);
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn two_dispatches_to_one_aggregate_sent_at_once_both_change_it() {
        let identifier: u32 = 1;
        let identity = repository::tests::fixture::Identity::from(identifier);
        let aggregate = repository::tests::fixture::Aggregate::from(identity);
        let aggregate_sequence = [aggregate];
        let repository = repository::tests::fixture::repository::Repository::from_iter(
            aggregate_sequence,
        );
        let service_repository = repository.clone();
        let service = fixture::Service::from(service_repository);
        let command_sequence: [fixture::Command; 2] = Default::default();
        let dispatch_future = |command: fixture::Command| {
            let dispatch = Dispatch::from((identity, command));
            service.try_dispatch_command(dispatch)
        };
        let dispatch_futures = command_sequence.map(dispatch_future);

        let outcome_sequence = repository::tests::helper::future_outputs(dispatch_futures);

        let aggregate_future = repository.try_read_aggregate(&identity);
        let aggregate_presence = repository::tests::helper::future_output(aggregate_future);
        let value_object_sequence: [u32; 2] = [1, 2];
        let expectation = value_object_sequence.map(Ok);
        let [.., value_object] = value_object_sequence;
        let stored_aggregate = repository::tests::fixture::Aggregate::from((
            identity,
            value_object,
        ));
        let presence = AggregatePresence::Presence {
            aggregate: stored_aggregate,
        };
        assert_eq!(outcome_sequence, expectation);
        let aggregate_presence_expectation = Ok(presence);
        assert_eq!(aggregate_presence, aggregate_presence_expectation);
    }
}
