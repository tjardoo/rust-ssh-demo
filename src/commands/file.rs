use clap::Subcommand;

use crate::utils::args::SharedServerArgs;

#[derive(Subcommand)]
pub enum FileCommand {
    /// Upload a file to the server
    Upload {
        /// The file to upload
        file: String,
        /// The destination path on the server
        destination: String,
        #[clap(flatten)]
        server_args: SharedServerArgs,
    },
    /// Download a file from the server
    Download {
        /// The file to download
        file: String,
        /// The destination path on the local machine
        destination: String,
        #[clap(flatten)]
        server_args: SharedServerArgs,
    },
}
