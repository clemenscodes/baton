pub(crate) mod request;

use crate::arguments::request::Request;
use clap::Parser;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Parser)]
#[command(
    name = "baton",
    version,
    about = "Runs a program inside the lock of a GPU."
)]
pub(crate) struct Arguments {
    #[arg(
        long,
        env = "BATON_ELEVATION_PROGRAM",
        default_value = "sudo",
        help = "The program that runs baton_host as root"
    )]
    elevation_program: PathBuf,
    #[arg(
        long,
        env = "BATON_ELEVATION_ARGUMENT",
        default_value = "-n",
        allow_hyphen_values = true,
        help = "An argument of the elevation program in front of baton_host"
    )]
    elevation_argument: Vec<String>,
    #[arg(
        long,
        env = "BATON_HOST_PROGRAM",
        default_value = "baton_host",
        help = "The path of baton_host"
    )]
    host_program: PathBuf,
    #[command(subcommand)]
    request: Request,
}

impl Layer for Arguments {
    type Layer = PresentationLayer;
}

impl Arguments {
    pub(crate) fn elevation_program(&self) -> &Path {
        &self.elevation_program
    }

    pub(crate) fn elevation_argument(&self) -> &[String] {
        &self.elevation_argument
    }

    pub(crate) fn host_program(&self) -> &Path {
        &self.host_program
    }

    pub(crate) fn request(&self) -> &Request {
        &self.request
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_arguments_hold_their_defaults_and_the_program_after_the_separator() {
        let arguments = Arguments::try_parse_from([
            "baton",
            "run",
            "--lock",
            "rtx3080",
            "--application",
            "resolve",
            "--",
            "davinci-resolve",
            "--render",
        ])
        .unwrap();

        assert_eq!(arguments.elevation_program(), Path::new("sudo"));
        assert_eq!(arguments.elevation_argument(), ["-n"]);
        assert_eq!(arguments.host_program(), Path::new("baton_host"));
        assert_eq!(
            arguments.request(),
            &Request::Run {
                lock: String::from("rtx3080"),
                application: String::from("resolve"),
                deadline: None,
                force: false,
                passthrough: false,
                program: vec![String::from("davinci-resolve"), String::from("--render")],
            }
        );
    }
}
