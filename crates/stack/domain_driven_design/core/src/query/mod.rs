mod handler;
mod local_handler;
mod synchronous_handler;

use crate::layer::ApplicationLayer;
use crate::layer::Layer;

pub use handler::QueryHandler;
pub use local_handler::LocalQueryHandler;
pub use synchronous_handler::SynchronousQueryHandler;

pub trait Query: Layer<Layer = ApplicationLayer> {
    type Output;
}
