#[derive(Clone)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub keys: Keys,
}

#[derive(Clone)]
pub struct Keys {
    pub public_key_path: String,
    pub private_key_path: String,
}

pub fn get_server_connection_detail(
    host: Option<String>,
    port: Option<u16>,
    user: Option<String>,
    public_key_path: Option<String>,
    private_key_path: Option<String>,
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

    let public_key_path = public_key_path.unwrap_or_else(|| {
        std::env::var("PUBLIC_KEY_PATH").expect("$PUBLIC_KEY_PATH is not set in `.env` file")
    });

    let private_key_path = private_key_path.unwrap_or_else(|| {
        std::env::var("PRIVATE_KEY_PATH").expect("$PRIVATE_KEY_PATH is not set in `.env` file")
    });

    Server {
        user,
        host,
        port,
        keys: Keys {
            public_key_path,
            private_key_path,
        },
    }
}
