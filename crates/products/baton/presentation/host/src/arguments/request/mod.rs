use clap::Subcommand;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Subcommand)]
pub(crate) enum Request {
    #[command(
        name = "enclose",
        about = "Puts a GPU under a lock and prints the device files it locks"
    )]
    Enclosure {
        #[arg(long, help = "The lock of the GPU, such as rtx3080")]
        lock: String,
        #[arg(long, help = "The PCI address of the GPU, such as 0000:01:00.0")]
        device: String,
    },
    #[command(name = "admit", about = "Makes an application eligible for a lock")]
    Admission {
        #[arg(long, help = "The lock of the GPU")]
        lock: String,
        #[arg(long, help = "The application that becomes eligible")]
        application: String,
        #[arg(
            long,
            help = "Makes the application the owner while no other owns the lock"
        )]
        default_owner: bool,
    },
    #[command(
        name = "dismiss",
        about = "Makes an application ineligible for a lock again"
    )]
    Dismissal {
        #[arg(long, help = "The lock of the GPU")]
        lock: String,
        #[arg(long, help = "The application that becomes ineligible")]
        application: String,
    },
    #[command(
        name = "acquire",
        about = "Records the demand of an application and returns once it owns the lock"
    )]
    Acquisition {
        #[arg(long, help = "The lock of the GPU")]
        lock: String,
        #[arg(long, help = "The eligible application the lock is acquired for")]
        application: String,
        #[arg(
            long,
            help = "The process that moves into the cgroup of the application"
        )]
        process: u32,
        #[arg(
            long,
            help = "The milliseconds to wait for the GPU at most, without it the acquisition waits as long as it takes"
        )]
        deadline: Option<u64>,
        #[arg(long, help = "Ends the users of the GPU instead of waiting for them")]
        force: bool,
        #[arg(long, help = "Hands the GPU to vfio-pci for a guest")]
        passthrough: bool,
    },
    #[command(name = "release", about = "Releases the lock of its owner")]
    Release {
        #[arg(long, help = "The lock of the GPU")]
        lock: String,
        #[arg(long, help = "The application that owns the lock")]
        application: String,
    },
    #[command(name = "refresh", about = "Finds the device files of every lock again")]
    Refresh,
    #[command(name = "remove", about = "Takes a GPU out of its lock")]
    Removal {
        #[arg(long, help = "The lock of the GPU")]
        lock: String,
    },
    #[command(
        name = "users",
        about = "Names every user of the GPU of a lock, every process and every console"
    )]
    Users {
        #[arg(long, help = "The lock of the GPU")]
        lock: String,
    },
    #[command(name = "status", about = "Prints the state of every lock")]
    Status,
}

impl Layer for Request {
    type Layer = PresentationLayer;
}
