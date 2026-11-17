mod arguments;
mod command_line;
mod host_adapters;

use crate::arguments::Arguments;
use crate::command_line::CommandLine;
use clap::Parser;
use std::io::{self, stderr, stdout};
use std::process::ExitCode;

fn main() -> io::Result<ExitCode> {
    let arguments = Arguments::parse();
    let command_line = CommandLine::from(arguments);
    let lock_administration = command_line.compose_administration();
    command_line.try_respond_exit_code(&lock_administration, stdout(), stderr())
}
