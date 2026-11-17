mod signal_presence;

pub use signal_presence::SignalPresence;

use std::io;

pub trait SignalTrap {
    fn try_arm_trap(&self) -> io::Result<()>;

    fn poll_signal_presence(&self) -> SignalPresence;

    fn lift_trap(&self);
}
