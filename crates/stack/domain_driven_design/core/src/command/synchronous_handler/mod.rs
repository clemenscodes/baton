use crate::layer::ApplicationLayer;
use crate::layer::Layer;

pub trait SynchronousCommandHandler<CommandType>: Layer<Layer = ApplicationLayer> {
    type Outcome;

    fn dispatch_command(&self, command: CommandType) -> Self::Outcome;
}
