use clap::{Parser, Subcommand};

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
}

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

#[derive(Subcommand)]
pub enum ActionCommand {
    /// Reboot the system
    Reboot(SharedServerArgs),
    /// Shutdown the system
    Shutdown(SharedServerArgs),
}

#[derive(Parser)]
pub struct SharedServerArgs {
    /// Optional hostname or IP address
    #[clap(long = "host")]
    pub host: Option<String>,
    /// Optional port number
    #[clap(long = "port")]
    pub port: Option<String>,
    /// Optional username
    #[clap(long = "user")]
    pub user: Option<String>,
}
