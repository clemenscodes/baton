use crate::Completion;
use std::io;

pub trait RunningProgram: Send {
    fn try_wait_completion(self) -> io::Result<Completion>;

    fn try_request_end(&mut self) -> io::Result<()>;

    fn try_poll_completion(&mut self) -> io::Result<Option<Completion>>;
}
