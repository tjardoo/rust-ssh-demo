use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ActionCommand {
    /// Reboot the system
    Reboot,
    /// Shutdown the system
    Shutdown,
    /// Show contents of a file
    Cat {
        /// The file to show
        file: String,
    },
}
