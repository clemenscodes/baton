use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use stack_lock_adapter_host_helper_elevation_program::HostHelper;
use stack_lock_adapter_program_runner_spawner::ProgramRunner;
use stack_lock_application::host_helper::HostHelper as HostHelperPort;
use stack_lock_application::lock::guard::ports::GuardPorts;
use stack_lock_application::program_runner::ProgramRunner as ProgramRunnerPort;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct GuardAdapters<SpawnerType, SignalTrapType> {
    host_helper: HostHelper<SpawnerType>,
    program_runner: ProgramRunner<SpawnerType, SignalTrapType>,
}

impl<SpawnerType, SignalTrapType> Layer for GuardAdapters<SpawnerType, SignalTrapType> {
    type Layer = PresentationLayer;
}

impl<SpawnerType, SignalTrapType>
    From<(
        HostHelper<SpawnerType>,
        ProgramRunner<SpawnerType, SignalTrapType>,
    )> for GuardAdapters<SpawnerType, SignalTrapType>
{
    fn from(
        (host_helper, program_runner): (
            HostHelper<SpawnerType>,
            ProgramRunner<SpawnerType, SignalTrapType>,
        ),
    ) -> Self {
        Self {
            host_helper,
            program_runner,
        }
    }
}

impl<SpawnerType, SignalTrapType> GuardPorts for GuardAdapters<SpawnerType, SignalTrapType>
where
    HostHelper<SpawnerType>: HostHelperPort,
    ProgramRunner<SpawnerType, SignalTrapType>: ProgramRunnerPort,
{
    type HostHelper = HostHelper<SpawnerType>;
    type ProgramRunner = ProgramRunner<SpawnerType, SignalTrapType>;

    fn host_helper(&self) -> &Self::HostHelper {
        &self.host_helper
    }

    fn program_runner(&self) -> &Self::ProgramRunner {
        &self.program_runner
    }
}
