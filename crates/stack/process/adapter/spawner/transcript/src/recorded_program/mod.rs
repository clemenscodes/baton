use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_process_core_invocation::{Completion, Invocation, RunningProgram};
use std::io;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct RecordedProgram {
    invocation: Invocation,
    completion: Completion,
    end_requests: Arc<Mutex<Vec<Invocation>>>,
}

impl Layer for RecordedProgram {
    type Layer = InfrastructureLayer;
}

impl From<(Invocation, Completion, Arc<Mutex<Vec<Invocation>>>)> for RecordedProgram {
    fn from(
        (invocation, completion, end_requests): (
            Invocation,
            Completion,
            Arc<Mutex<Vec<Invocation>>>,
        ),
    ) -> Self {
        Self {
            invocation,
            completion,
            end_requests,
        }
    }
}

impl RunningProgram for RecordedProgram {
    fn try_wait_completion(self) -> io::Result<Completion> {
        Ok(self.completion)
    }

    fn try_request_end(&mut self) -> io::Result<()> {
        let mut end_requests = match self.end_requests.lock() {
            Ok(end_requests) => end_requests,
            Err(poison) => poison.into_inner(),
        };
        end_requests.push(self.invocation.clone());
        Ok(())
    }

    fn try_poll_completion(&mut self) -> io::Result<Option<Completion>> {
        Ok(Some(self.completion.clone()))
    }
}
