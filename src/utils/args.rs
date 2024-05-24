use clap::Parser;

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
