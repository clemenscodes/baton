mod arguments;
mod command_line;
mod guard_adapters;

use crate::arguments::Arguments;
use crate::command_line::CommandLine;
use clap::Parser;
use stack_process_adapter_spawner_standard_library::Spawner;
use std::io::{self, stderr, stdout};
use std::process::ExitCode;

fn main() -> io::Result<ExitCode> {
    let arguments = Arguments::parse();
    let command_line = CommandLine::from(arguments);
    let spawners = (Spawner::from(()), Spawner::from(()));
    command_line.try_respond_exit_code(spawners, stdout(), stderr())
}
