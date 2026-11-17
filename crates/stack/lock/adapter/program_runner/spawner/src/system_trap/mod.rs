use crate::signal_trap::{SignalPresence, SignalTrap};
use signal_hook::SigId;
use signal_hook::consts::signal::{SIGHUP, SIGINT, SIGTERM};
use signal_hook::flag;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Debug, Default)]
pub struct SystemTrap {
    signal_flag: Arc<AtomicBool>,
    lift_flag: Arc<AtomicBool>,
}

impl Layer for SystemTrap {
    type Layer = InfrastructureLayer;
}

impl From<()> for SystemTrap {
    fn from(_: ()) -> Self {
        let signal_flag = Arc::new(AtomicBool::new(false));
        let lift_flag = Arc::new(AtomicBool::new(false));
        Self {
            signal_flag,
            lift_flag,
        }
    }
}

impl SystemTrap {
    const TRAP_SIGNALS: [i32; 3] = [SIGTERM, SIGINT, SIGHUP];
}

impl SignalTrap for SystemTrap {
    fn try_arm_trap(&self) -> io::Result<()> {
        let registrations = Self::TRAP_SIGNALS.map(|trap_signal| {
            let lift_flag = Arc::clone(&self.lift_flag);
            let signal_flag = Arc::clone(&self.signal_flag);
            flag::register_conditional_default(trap_signal, lift_flag)
                .and_then(|_| flag::register(trap_signal, signal_flag))
        });
        let registration_outcome: io::Result<Vec<SigId>> = registrations.into_iter().collect();
        registration_outcome.map(drop)
    }

    fn poll_signal_presence(&self) -> SignalPresence {
        match self.signal_flag.swap(false, Ordering::SeqCst) {
            true => SignalPresence::Presence,
            false => SignalPresence::Absence,
        }
    }

    fn lift_trap(&self) {
        self.lift_flag.store(true, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use signal_hook::low_level;

    #[test]
    fn an_armed_trap_catches_every_signal_of_an_end_once_until_it_is_lifted() {
        let system_trap = SystemTrap::from(());

        let registration_outcome = system_trap.try_arm_trap();
        let signal_presence = system_trap.poll_signal_presence();
        let signal_presences = SystemTrap::TRAP_SIGNALS.map(|trap_signal| {
            low_level::raise(trap_signal)
                .map(|_| {
                    [
                        system_trap.poll_signal_presence(),
                        system_trap.poll_signal_presence(),
                    ]
                })
                .ok()
        });
        system_trap.lift_trap();

        let expectation = Some([SignalPresence::Presence, SignalPresence::Absence]);
        assert!(registration_outcome.is_ok());
        assert_eq!(signal_presence, SignalPresence::Absence);
        assert_eq!(signal_presences, [expectation; 3]);
        assert!(system_trap.lift_flag.load(Ordering::SeqCst));
    }
}
