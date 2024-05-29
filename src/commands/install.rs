use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum InstallCommand {
    /// Install Nginx
    Nginx,
}
