use crate::layer::ApplicationLayer;
use crate::layer::Layer;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Dispatch<Identity, CommandType> {
    identity: Identity,
    command: CommandType,
}

impl<Identity, CommandType> From<(Identity, CommandType)> for Dispatch<Identity, CommandType> {
    fn from((identity, command): (Identity, CommandType)) -> Self {
        Self { identity, command }
    }
}

impl<Identity, CommandType> Layer for Dispatch<Identity, CommandType> {
    type Layer = ApplicationLayer;
}

impl<Identity, CommandType> Dispatch<Identity, CommandType> {
    pub fn identity(&self) -> Identity
    where
        Identity: Clone,
    {
        self.identity.clone()
    }

    pub fn command(self) -> CommandType {
        self.command
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dispatch_keeps_the_identity_it_is_made_of() {
        let identity: u32 = 4;
        let command = "raise the counter";
        let dispatch = Dispatch::from((identity, command));

        let dispatch_identity = dispatch.identity();

        assert_eq!(dispatch_identity, identity);
    }

    #[test]
    fn a_dispatch_keeps_the_command_it_is_made_of() {
        let identity: u32 = 4;
        let expectation = "raise the counter";
        let dispatch = Dispatch::from((identity, expectation));

        let command = dispatch.command();

        assert_eq!(command, expectation);
    }
}
