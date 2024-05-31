use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum FileCommand {
    /// Show contents of a file
    View {
        /// The file to show
        file: String,
    },
}
