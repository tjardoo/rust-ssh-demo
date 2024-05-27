use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InfoCommand {
    /// Get hardware information
    Hardware,
    /// Get memory information
    Memory,
    /// Get partition information
    Partitions,
    /// Get CPU information
    Cpu,
    /// Get temperature information
    Temperature,
    /// Get uptime information
    Uptime,
    /// Get version information
    Version,
    /// Get current directory information
    CurrentDir,
}
