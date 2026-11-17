use crate::signal_call::{SignalCall, SignalKind};
use rustix::process::{Pid, Signal};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct SystemSignal;

impl Layer for SystemSignal {
    type Layer = InfrastructureLayer;
}

impl SignalCall for SystemSignal {
    fn try_send_signal(&self, process_identifier: u32, signal_kind: SignalKind) -> io::Result<()> {
        let kernel_signal = match signal_kind {
            SignalKind::Termination => Signal::TERM,
            SignalKind::Kill => Signal::KILL,
        };
        i32::try_from(process_identifier)
            .ok()
            .and_then(Pid::from_raw)
            .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))
            .and_then(|signal_target| {
                rustix::process::kill_process(signal_target, kernel_signal).map_err(io::Error::from)
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_signal_to_a_process_that_cannot_exist_is_refused_by_the_kernel() {
        let system_signal = SystemSignal;

        let outcome = [SignalKind::Termination, SignalKind::Kill]
            .map(|signal_kind| system_signal.try_send_signal(4999999, signal_kind));
        let identifier_outcome = system_signal.try_send_signal(0, SignalKind::Termination);

        assert!(outcome.iter().all(|outcome| {
            outcome
                .as_ref()
                .is_err_and(|error| error.raw_os_error() == Some(3))
        }));
        assert!(identifier_outcome.is_err_and(|error| error.kind() == io::ErrorKind::InvalidInput));
    }
}
