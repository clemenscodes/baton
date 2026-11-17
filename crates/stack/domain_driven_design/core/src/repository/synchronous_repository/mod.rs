use crate::aggregate_root::AggregateRoot;
use crate::command::Command;
use crate::dispatch::Dispatch;
use crate::repository::Addition;
use crate::repository::AggregatePresence;
use crate::repository::Aggregates;
use crate::repository::Identities;
use crate::repository::OutcomePresence;

pub trait SynchronousRepository<Aggregate: AggregateRoot> {
    type Error;

    fn try_read_aggregate(
        &self,
        identity: &Aggregate::Identity,
    ) -> Result<AggregatePresence<Aggregate>, Self::Error>;

    fn try_read_aggregates(
        &self,
        identities: &Identities<Aggregate::Identity>,
    ) -> Result<Aggregates<Aggregate>, Self::Error>;

    fn try_add_aggregate(&self, aggregate: &Aggregate) -> Result<Addition, Self::Error>;

    fn try_save_aggregate(&self, aggregate: &Aggregate) -> Result<(), Self::Error>;

    fn try_change_aggregate<CommandType>(
        &self,
        dispatch: Dispatch<Aggregate::Identity, CommandType>,
    ) -> Result<OutcomePresence<CommandType::Outcome>, Self::Error>
    where
        CommandType: Command<Aggregate> + Send + 'static,
        CommandType::Outcome: Send + 'static;

    fn try_delete_aggregate(
        &self,
        identity: &Aggregate::Identity,
    ) -> Result<AggregatePresence<Aggregate>, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository;

    #[test]
    fn an_addition_under_a_free_identity_keeps_the_aggregate_and_is_an_insertion() {
        let identifier: u32 = 1;
        let identity = repository::tests::fixture::Identity::from(identifier);
        let value_object: u32 = 2;
        let aggregate = repository::tests::fixture::Aggregate::from((identity, value_object));
        let synchronous_repository = repository::tests::fixture::repository::Repository::default();

        let addition = synchronous_repository.try_add_aggregate(&aggregate);

        let aggregate_presence = synchronous_repository.try_read_aggregate(&identity);
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
        let identity = repository::tests::fixture::Identity::from(identifier);
        let stored_aggregate = repository::tests::fixture::Aggregate::from(identity);
        let value_object: u32 = 7;
        let aggregate = repository::tests::fixture::Aggregate::from((identity, value_object));
        let aggregate_sequence = [stored_aggregate];
        let synchronous_repository = repository::tests::fixture::repository::Repository::from_iter(
            aggregate_sequence,
        );

        let addition = synchronous_repository.try_add_aggregate(&aggregate);

        let aggregate_presence = synchronous_repository.try_read_aggregate(&identity);
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
