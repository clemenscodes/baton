use clap::Subcommand;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Subcommand)]
pub(crate) enum Request {
    #[command(about = "Runs a program inside the lock of a GPU and releases the lock when it ends")]
    Run {
        #[arg(long, help = "The lock of the GPU, such as rtx3080")]
        lock: String,
        #[arg(long, help = "The eligible application the lock is acquired for")]
        application: String,
        #[arg(
            long,
            help = "The milliseconds to wait for the GPU at most, without it the run waits as long as it takes"
        )]
        deadline: Option<u64>,
        #[arg(long, help = "Ends the users of the GPU instead of waiting for them")]
        force: bool,
        #[arg(
            long,
            help = "Hands the GPU to vfio-pci for a guest before the program starts"
        )]
        passthrough: bool,
        #[arg(
            last = true,
            required = true,
            help = "The program and its arguments, after --"
        )]
        program: Vec<String>,
    },
    #[command(about = "Prints the state of every lock")]
    Status,
}

impl Layer for Request {
    type Layer = PresentationLayer;
}
