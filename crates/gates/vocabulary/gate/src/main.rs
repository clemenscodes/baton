mod domain;

use crate::domain::contexts::Contexts;
use clap::Parser;
use gates_vocabulary_language_adapter_command_line::{CommandLine, Session};
use std::io::{self, stderr, stdout};
use std::process::ExitCode;

fn main() -> io::Result<ExitCode> {
    let command_line = CommandLine::parse();
    let contexts = Contexts;
    let report = stdout();
    let errors = stderr();
    let session = Session::from((contexts, report, errors));
    command_line.try_respond_exit_code(session)
}
