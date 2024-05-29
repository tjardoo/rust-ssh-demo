use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum ControlCommand {
    /// Update the system
    Update,
}
