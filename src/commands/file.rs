use clap::Subcommand;

use crate::utils::args::SharedServerArgs;

#[derive(Subcommand)]
pub enum FileCommand {
    /// Upload a file
    Upload(SharedServerArgs),
    /// Download a file
    Download(SharedServerArgs),
}
