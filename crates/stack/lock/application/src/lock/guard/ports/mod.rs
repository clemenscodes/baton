use crate::host_helper::HostHelper;
use crate::program_runner::ProgramRunner;

pub trait GuardPorts {
    type HostHelper: HostHelper;
    type ProgramRunner: ProgramRunner;

    fn host_helper(&self) -> &Self::HostHelper;

    fn program_runner(&self) -> &Self::ProgramRunner;
}
