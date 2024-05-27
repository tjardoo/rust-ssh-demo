use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ActionCommand {
    /// Reboot the system
    Reboot,
    /// Shutdown the system
    Shutdown,
}
