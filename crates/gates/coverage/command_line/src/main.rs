mod mode;
mod services;

use mode::Mode;
use services::Services;
use std::env::args_os;
use std::io::{self, stderr, stdin, stdout};
use std::process::ExitCode;

fn main() -> io::Result<ExitCode> {
    let arguments = Vec::from_iter(args_os().skip(1));
    let mode = Mode::from(arguments);
    let input = stdin();
    let output = stdout();
    let errors = stderr();
    let services = Services::from((input, output, errors));
    services.try_respond_exit_code(mode)
}
