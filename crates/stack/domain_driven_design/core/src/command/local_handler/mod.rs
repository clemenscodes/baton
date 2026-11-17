use crate::layer::ApplicationLayer;
use crate::layer::Layer;
use std::future::Future;

pub trait LocalCommandHandler<CommandType>: Layer<Layer = ApplicationLayer> {
    type Outcome;

    fn dispatch_command(&self, command: CommandType) -> impl Future<Output = Self::Outcome>;
}
