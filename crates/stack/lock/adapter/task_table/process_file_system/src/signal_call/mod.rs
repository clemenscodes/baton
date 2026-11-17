use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum SignalKind {
    Termination,
    Kill,
}

pub trait SignalCall {
    fn try_send_signal(&self, process_identifier: u32, signal_kind: SignalKind) -> io::Result<()>;
}
