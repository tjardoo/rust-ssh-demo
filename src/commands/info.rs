use clap::Subcommand;

use crate::utils::args::SharedServerArgs;

#[derive(Subcommand)]
pub enum InfoCommand {
    /// Get hardware information
    Hardware(SharedServerArgs),
    /// Get memory information
    Memory(SharedServerArgs),
    /// Get partition information
    Partitions(SharedServerArgs),
    /// Get temperature information
    Temperature(SharedServerArgs),
}
