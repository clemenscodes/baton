use crate::aggregate_root::AggregateRoot;
use crate::application_service::ApplicationService;
use crate::command::Command;
use crate::dispatch::Dispatch;
use crate::entity::Entity;
use crate::repository::OutcomePresence;
use crate::repository::SynchronousRepository;
use crate::service::Error;

type SynchronousServiceError<ServiceType, Aggregate> = Error<
    <Aggregate as Entity>::Identity,
    <<ServiceType as SynchronousService<Aggregate>>::Repository as SynchronousRepository<
        Aggregate,
    >>::Error,
>;

pub trait SynchronousService<Aggregate: AggregateRoot>: ApplicationService {
    type Repository: SynchronousRepository<Aggregate>;

    fn repository(&self) -> &Self::Repository;

    fn try_dispatch_command<CommandType>(
        &self,
        dispatch: Dispatch<Aggregate::Identity, CommandType>,
    ) -> Result<CommandType::Outcome, SynchronousServiceError<Self, Aggregate>>
    where
        CommandType: Command<Aggregate> + Send + 'static,
        CommandType::Outcome: Send + 'static,
    {
        let repository = self.repository();
        let identity = dispatch.identity();
        let outcome_presence = repository.try_change_aggregate(dispatch)?;
        match outcome_presence {
            OutcomePresence::Presence { outcome } => Ok(outcome),
            OutcomePresence::Absence => {
                let absence = Error::Absence { identity };
                Err(absence)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository;
    use crate::repository::AggregatePresence;
    use crate::repository::Aggregates;
    use crate::repository::Identities;
    use crate::service;

    #[test]
    fn a_dispatch_changes_the_stored_aggregate_and_returns_the_outcome() {
        let identifier: u32 = 1;
        let identity = repository::tests::fixture::Identity::from(identifier);
        let aggregate = repository::tests::fixture::Aggregate::from(identity);
        let aggregate_sequence = [aggregate];
        let synchronous_repository = repository::tests::fixture::repository::Repository::from_iter(
            aggregate_sequence,
        );
        let service_repository = synchronous_repository.clone();
        let synchronous_service = service::tests::fixture::SynchronousService::from(
            service_repository,
        );
        let command = service::tests::fixture::Command;
        let dispatch = Dispatch::from((identity, command));

        let outcome = synchronous_service.try_dispatch_command(dispatch);

        let aggregate_presence = synchronous_repository.try_read_aggregate(&identity);
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
        let synchronous_repository = repository::tests::fixture::repository::Repository::default();
        let synchronous_service = service::tests::fixture::SynchronousService::from(
            synchronous_repository,
        );
        let command = service::tests::fixture::Command;
        let dispatch = Dispatch::from((identity, command));

        let outcome = synchronous_service.try_dispatch_command(dispatch);

        let absence = Error::Absence { identity };
        let expectation = Err(absence);
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn a_dispatch_leaves_every_aggregate_of_another_identity_as_it_was() {
        let identifier_sequence: [u32; 2] = [1, 2];
        let identity_sequence = identifier_sequence.map(repository::tests::fixture::Identity::from);
        let aggregate_sequence = identity_sequence.map(repository::tests::fixture::Aggregate::from);
        let synchronous_repository = repository::tests::fixture::repository::Repository::from_iter(
            aggregate_sequence,
        );
        let service_repository = synchronous_repository.clone();
        let synchronous_service = service::tests::fixture::SynchronousService::from(
            service_repository,
        );
        let command = service::tests::fixture::Command;
        let [identity, ..] = identity_sequence;
        let dispatch = Dispatch::from((identity, command));

        let outcome = synchronous_service.try_dispatch_command(dispatch);

        let identities = Identities::from_iter(identity_sequence);
        let aggregates = synchronous_repository.try_read_aggregates(&identities);
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
        let synchronous_service = service::tests::fixture::SynchronousService::from(
            error_repository,
        );
        let command = service::tests::fixture::Command;
        let dispatch = Dispatch::from((identity, command));

        let outcome = synchronous_service.try_dispatch_command(dispatch);

        let repository_error = repository::tests::fixture::Error;
        let error = Error::Repository {
            error: repository_error,
        };
        let expectation = Err(error);
        assert_eq!(outcome, expectation);
    }
}
