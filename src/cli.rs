use clap::{Parser, Subcommand};

use crate::commands::{
    action::ActionCommand, control::ControlCommand, file::FileCommand, info::InfoCommand,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
    #[arg(long)]
    pub host: Option<String>,
    #[arg(long)]
    pub port: Option<u16>,
    #[arg(long)]
    pub user: Option<String>,
    #[arg(long = "public-key")]
    pub public_key_path: Option<String>,
    #[arg(long = "private-key")]
    pub private_key_path: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Test connection
    Test,
    #[clap(subcommand)]
    /// Get system information
    Info(InfoCommand),
    /// Perform a system action
    #[clap(subcommand)]
    Action(ActionCommand),
    /// Perform a file operation
    #[clap(subcommand)]
    File(FileCommand),
    /// Perform a system control operation
    #[clap(subcommand)]
    Control(ControlCommand),
}
