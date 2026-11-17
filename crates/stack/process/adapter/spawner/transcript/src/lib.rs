mod recorded_program;

pub use recorded_program::RecordedProgram;

use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_process_core_invocation::{Completion, Invocation, Spawner as SpawnerPort};
use std::collections::VecDeque;
use std::io;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct Spawner {
    invocations: Mutex<Vec<Invocation>>,
    completions: Mutex<VecDeque<Option<Completion>>>,
    end_requests: Arc<Mutex<Vec<Invocation>>>,
}

impl Layer for Spawner {
    type Layer = InfrastructureLayer;
}

impl Adapter for Spawner {}

impl Port for Spawner {}

impl DrivenPort for Spawner {}

impl From<Vec<Option<Completion>>> for Spawner {
    fn from(completion_sequence: Vec<Option<Completion>>) -> Self {
        let invocations = Mutex::new(Vec::new());
        let completion_queue = VecDeque::from(completion_sequence);
        let completions = Mutex::new(completion_queue);
        let end_requests = Arc::new(Mutex::new(Vec::new()));
        Self {
            invocations,
            completions,
            end_requests,
        }
    }
}

impl From<()> for Spawner {
    fn from(_: ()) -> Self {
        let completion_sequence = Vec::new();
        Self::from(completion_sequence)
    }
}

impl SpawnerPort for Spawner {
    type RunningProgram = RecordedProgram;

    fn try_spawn_invocation(&self, invocation: &Invocation) -> io::Result<Completion> {
        self.try_record_invocation(invocation)
    }

    fn try_start_invocation(&self, invocation: &Invocation) -> io::Result<RecordedProgram> {
        let completion = self.try_record_invocation(invocation)?;
        let end_requests = Arc::clone(&self.end_requests);
        let recorded_program = RecordedProgram::from((
            invocation.clone(),
            completion,
            end_requests,
        ));
        Ok(recorded_program)
    }
}

impl Spawner {
    pub fn invocations(&self) -> Vec<Invocation> {
        match self.invocations.lock() {
            Ok(invocations) => invocations.clone(),
            Err(poison) => poison.into_inner().clone(),
        }
    }

    pub fn end_requests(&self) -> Vec<Invocation> {
        match self.end_requests.lock() {
            Ok(end_requests) => end_requests.clone(),
            Err(poison) => poison.into_inner().clone(),
        }
    }

    fn try_record_invocation(&self, invocation: &Invocation) -> io::Result<Completion> {
        let mut invocations = match self.invocations.lock() {
            Ok(invocations) => invocations,
            Err(poison) => poison.into_inner(),
        };
        invocations.push(invocation.clone());
        let mut completions = match self.completions.lock() {
            Ok(completions) => completions,
            Err(poison) => poison.into_inner(),
        };
        let maybe_completion = completions.pop_front();
        match maybe_completion {
            Some(Some(completion)) => Ok(completion),
            Some(None) => Err(io::Error::from(io::ErrorKind::NotFound)),
            None => {
                let output = Vec::new();
                Ok(Completion::Success { output })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stack_process_core_invocation::{RunningProgram, Stream};
    use std::ffi::OsString;

    mod fixture {
        use super::*;

        pub(crate) fn invocation_factory(program: &str) -> Invocation {
            let arguments = Vec::new();
            Invocation::from((OsString::from(program), arguments, Stream::Capture))
        }
    }

    #[test]
    fn a_recording_spawner_hands_out_its_completions_in_order_and_records_every_invocation() {
        let status = String::from("exit status: 2");
        let failure = Completion::Failure {
            status,
            maybe_code: None,
        };
        let completion_sequence = vec![Some(failure.clone()), None];
        let spawner = Spawner::from(completion_sequence);
        let failure_invocation = fixture::invocation_factory("ip");
        let absence_invocation = fixture::invocation_factory("ssh");
        let success_invocation = fixture::invocation_factory("scp");

        let failure_outcome = spawner.try_spawn_invocation(&failure_invocation);
        let absence_outcome = spawner.try_start_invocation(&absence_invocation);
        let success_outcome = spawner
            .try_start_invocation(&success_invocation)
            .and_then(|recorded_program| recorded_program.try_wait_completion());

        let output = Vec::new();
        let success = Completion::Success { output };
        let absence_kind = absence_outcome.err().map(|error| error.kind());
        assert_eq!(failure_outcome.ok(), Some(failure));
        assert_eq!(absence_kind, Some(io::ErrorKind::NotFound));
        assert_eq!(success_outcome.ok(), Some(success));
        assert_eq!(
            spawner.invocations(),
            vec![failure_invocation, absence_invocation, success_invocation]
        );
    }

    #[test]
    fn a_recorded_program_records_every_request_to_end_and_polls_its_completion() {
        let spawner = Spawner::from(());
        let invocation = fixture::invocation_factory("framelink");

        let mut recorded_program = spawner.try_start_invocation(&invocation).unwrap();
        let end_outcome = recorded_program.try_request_end();
        let poll_outcome = recorded_program.try_poll_completion();

        let output = Vec::new();
        assert!(end_outcome.is_ok());
        assert_eq!(
            poll_outcome.ok(),
            Some(Some(Completion::Success { output }))
        );
        assert_eq!(spawner.end_requests(), vec![invocation]);
    }

    #[test]
    fn a_spawner_without_completions_succeeds_with_an_empty_output() {
        let spawner = Spawner::from(());
        let invocation = fixture::invocation_factory("qemu-img");

        let completion = spawner.try_spawn_invocation(&invocation).unwrap();

        let output = Vec::new();
        assert_eq!(completion, Completion::Success { output });
    }

    #[test]
    fn a_spawner_whose_locks_are_poisoned_still_records_and_answers() {
        let spawner = Spawner::from(());
        let invocation = fixture::invocation_factory("ip");

        let poison_outcome = std::thread::scope(|scope| {
            scope
                .spawn(|| {
                    let _invocations = spawner.invocations.lock();
                    let _completions = spawner.completions.lock();
                    panic!("the lock holder fails.");
                })
                .join()
        });
        let outcome = spawner.try_spawn_invocation(&invocation);

        let output = Vec::new();
        let success = Completion::Success { output };
        assert!(poison_outcome.is_err());
        assert_eq!(outcome.ok(), Some(success));
        assert_eq!(spawner.invocations(), vec![invocation]);
    }
}
