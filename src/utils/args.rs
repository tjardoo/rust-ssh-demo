use clap::Parser;

#[derive(Parser, Clone)]
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

#[derive(Parser)]
pub struct Server {
    pub user: String,
    pub host: String,
    pub port: String,
}

pub fn get_server_connection_detail(args: SharedServerArgs) -> Server {
    let host = args
        .host
        .unwrap_or_else(|| std::env::var("HOST").expect("$HOST is not set in `.env` file"));

    let port = args
        .port
        .unwrap_or_else(|| std::env::var("PORT").expect("$PORT is not set in `.env` file"));

    let user = args
        .user
        .unwrap_or_else(|| std::env::var("USER").expect("$USER is not set in `.env` file"));

    Server { user, host, port }
}
