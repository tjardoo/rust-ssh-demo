use clap::Subcommand;

use crate::utils::args::SharedServerArgs;

#[derive(Subcommand)]
pub enum ActionCommand {
    /// Reboot the system
    Reboot(SharedServerArgs),
    /// Shutdown the system
    Shutdown(SharedServerArgs),
}
