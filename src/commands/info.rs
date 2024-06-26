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
    Pwd,
    /// Get disk usage information for a directory
    Du {
        /// The directory to check
        directory: String,
    },
    /// Check if a service is installed
    ServiceInstalledCheck {
        /// The service to check
        service: String,
    },
}
