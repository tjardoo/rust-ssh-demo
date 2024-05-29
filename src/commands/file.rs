use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum FileCommand {
    /// Upload a file to the server
    Upload {
        /// The file to upload
        file: String,
        /// The destination path on the server
        destination: String,
    },
    /// Download a file from the server
    Download {
        /// The file to download
        file: String,
        /// The destination path on the local machine
        destination: String,
    },
    /// Show contents of a file
    View {
        /// The file to show
        file: String,
    },
}
