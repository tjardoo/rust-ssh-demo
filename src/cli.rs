use clap::{Parser, Subcommand};

use crate::{
    commands::{action::ActionCommand, file::FileCommand, info::InfoCommand},
    utils::args::SharedServerArgs,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Test connection
    Test(SharedServerArgs),
    #[clap(subcommand)]
    /// Get system information
    Info(InfoCommand),
    /// Perform a system action
    #[clap(subcommand)]
    Action(ActionCommand),
    /// Perform a file operation
    #[clap(subcommand)]
    File(FileCommand),
}
