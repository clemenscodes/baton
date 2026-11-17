use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_process_core_invocation::{Completion, RunningProgram};
use std::io;
use std::process::{Child, ExitStatus, Output};

#[derive(Debug)]
pub struct ChildProgram {
    child: Child,
}

impl Layer for ChildProgram {
    type Layer = InfrastructureLayer;
}

impl From<Child> for ChildProgram {
    fn from(child: Child) -> Self {
        Self { child }
    }
}

impl RunningProgram for ChildProgram {
    fn try_wait_completion(self) -> io::Result<Completion> {
        self.try_read_completion()
    }

    fn try_request_end(&mut self) -> io::Result<()> {
        let maybe_status = self.child.try_wait()?;
        match maybe_status {
            Some(_) => Ok(()),
            None => Self::try_send_termination(&self.child),
        }
    }

    fn try_poll_completion(&mut self) -> io::Result<Option<Completion>> {
        let maybe_status = self.child.try_wait()?;
        let maybe_completion = maybe_status.map(|exit_status| {
            let output = Vec::new();
            Self::compose_status_completion(exit_status, output)
        });
        Ok(maybe_completion)
    }
}

impl ChildProgram {
    pub(crate) fn try_read_completion(self) -> io::Result<Completion> {
        let Self { child } = self;
        child.wait_with_output().map(Self::compose_completion)
    }

    #[cfg(unix)]
    fn try_send_termination(child: &Child) -> io::Result<()> {
        rustix::process::kill_process(
            rustix::process::Pid::from_child(child),
            rustix::process::Signal::TERM,
        )
        .map_err(io::Error::from)
    }

    #[cfg(not(unix))]
    fn try_send_termination(_child: &Child) -> io::Result<()> {
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }

    fn compose_completion(child_output: Output) -> Completion {
        Self::compose_status_completion(child_output.status, child_output.stdout)
    }

    fn compose_status_completion(exit_status: ExitStatus, output: Vec<u8>) -> Completion {
        match exit_status.success() {
            true => Completion::Success { output },
            false => {
                let status_text = exit_status.to_string();
                let maybe_code = exit_status.code();
                Completion::Failure {
                    status: status_text,
                    maybe_code,
                }
            }
        }
    }
}
