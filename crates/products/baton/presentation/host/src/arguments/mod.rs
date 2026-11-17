pub(crate) mod request;

use crate::arguments::request::Request;
use clap::Parser;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Parser)]
#[command(
    name = "baton_host",
    version,
    about = "Administers the locks of the GPUs as root."
)]
pub(crate) struct Arguments {
    #[arg(
        long,
        env = "BATON_HIERARCHY_DIRECTORY",
        default_value = "/sys/fs/cgroup",
        help = "The mount of the unified cgroup hierarchy"
    )]
    hierarchy_directory: PathBuf,
    #[arg(
        long,
        env = "BATON_SYSTEM_DIRECTORY",
        default_value = "/",
        help = "The root below which sys, proc and dev are read"
    )]
    system_directory: PathBuf,
    #[arg(
        long,
        env = "BATON_RECORD_DIRECTORY",
        default_value = "/run/baton",
        help = "The directory of the records of the locks"
    )]
    record_directory: PathBuf,
    #[arg(
        long,
        env = "BATON_PIN_DIRECTORY",
        default_value = "/sys/fs/bpf/baton",
        help = "The directory in bpffs where the device program and its maps are pinned"
    )]
    pin_directory: PathBuf,
    #[arg(
        long,
        env = "BATON_ANCESTRY_DEPTH",
        default_value_t = 16,
        help = "The depth of the cgroups the device program checks for an owner"
    )]
    ancestry_depth: u32,
    #[arg(
        long,
        env = "BATON_CHECK_INTERVAL",
        default_value_t = 200,
        help = "The milliseconds between two checks of a wait"
    )]
    check_interval: u64,
    #[arg(
        long,
        env = "BATON_END_DEADLINE",
        default_value_t = 10000,
        help = "The milliseconds a process has between SIGTERM and SIGKILL"
    )]
    end_deadline: u64,
    #[arg(
        long,
        env = "BATON_BINDING_DEADLINE",
        default_value_t = 10000,
        help = "The milliseconds a driver has to bind or to release a GPU"
    )]
    binding_deadline: u64,
    #[arg(
        long,
        env = "BATON_CONSOLE_PROGRAM",
        default_value = "con2fbmap",
        help = "The program that maps a console to a framebuffer"
    )]
    console_program: PathBuf,
    #[arg(
        long,
        env = "BATON_PERSISTENCE_PROGRAM",
        default_value = "nvidia-smi",
        help = "The program that sets the persistence of an NVIDIA GPU"
    )]
    persistence_program: PathBuf,
    #[command(subcommand)]
    request: Request,
}

impl Layer for Arguments {
    type Layer = PresentationLayer;
}

impl Arguments {
    pub(crate) fn hierarchy_directory(&self) -> &Path {
        &self.hierarchy_directory
    }

    pub(crate) fn system_directory(&self) -> &Path {
        &self.system_directory
    }

    pub(crate) fn record_directory(&self) -> &Path {
        &self.record_directory
    }

    pub(crate) fn pin_directory(&self) -> &Path {
        &self.pin_directory
    }

    pub(crate) fn ancestry_depth(&self) -> u32 {
        self.ancestry_depth
    }

    pub(crate) fn check_interval(&self) -> u64 {
        self.check_interval
    }

    pub(crate) fn end_deadline(&self) -> u64 {
        self.end_deadline
    }

    pub(crate) fn binding_deadline(&self) -> u64 {
        self.binding_deadline
    }

    pub(crate) fn console_program(&self) -> &Path {
        &self.console_program
    }

    pub(crate) fn persistence_program(&self) -> &Path {
        &self.persistence_program
    }

    pub(crate) fn request(&self) -> &Request {
        &self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arguments_hold_their_defaults_and_the_request() {
        let arguments = Arguments::try_parse_from(["baton_host", "users", "--lock", "rtx3080"])
            .unwrap();

        assert_eq!(arguments.hierarchy_directory(), Path::new("/sys/fs/cgroup"));
        assert_eq!(arguments.system_directory(), Path::new("/"));
        assert_eq!(arguments.record_directory(), Path::new("/run/baton"));
        assert_eq!(arguments.pin_directory(), Path::new("/sys/fs/bpf/baton"));
        assert_eq!(arguments.ancestry_depth(), 16);
        assert_eq!(arguments.check_interval(), 200);
        assert_eq!(arguments.end_deadline(), 10000);
        assert_eq!(arguments.binding_deadline(), 10000);
        assert_eq!(arguments.console_program(), Path::new("con2fbmap"));
        assert_eq!(arguments.persistence_program(), Path::new("nvidia-smi"));
        assert_eq!(
            arguments.request(),
            &Request::Users {
                lock: String::from("rtx3080"),
            }
        );
    }
}
