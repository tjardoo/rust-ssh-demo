#[derive(Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub user: String,
}

pub fn get_server_connection_detail(
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
) -> Server {
    let host =
        host.unwrap_or_else(|| std::env::var("HOST").expect("$HOST is not set in `.env` file"));

    let port: u16 = port.unwrap_or_else(|| {
        std::env::var("PORT")
            .expect("$PORT is not set in `.env` file")
            .parse()
            .unwrap()
    });

    let user =
        user.unwrap_or_else(|| std::env::var("USER").expect("$USER is not set in `.env` file"));

    Server { user, host, port }
}
