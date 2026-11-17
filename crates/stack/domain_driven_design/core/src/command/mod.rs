mod handler;
mod local_handler;
mod synchronous_handler;

use crate::aggregate_root::AggregateRoot;
use crate::change::Change;
use crate::layer::ApplicationLayer;
use crate::layer::Layer;

pub use handler::CommandHandler;
pub use local_handler::LocalCommandHandler;
pub use synchronous_handler::SynchronousCommandHandler;

pub trait Command<Aggregate: AggregateRoot>: Layer<Layer = ApplicationLayer> {
    type Outcome;

    fn change_aggregate(self, aggregate: Aggregate) -> Change<Aggregate, Self::Outcome>;
}
