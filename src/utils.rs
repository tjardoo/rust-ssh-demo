use std::fmt::Display;

use crate::cli::Cli;

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

pub struct ServerCommand {
    pub command: String,
    pub print_output: bool,
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{}:{}", self.user, self.host, self.port)
    }
}

pub fn get_server_information(cli: &Cli) -> Server {
    let host = cli
        .host
        .clone()
        .unwrap_or_else(|| std::env::var("HOST").expect("$HOST is not set in `.env` file"));

    let port: u16 = cli.port.unwrap_or_else(|| {
        std::env::var("PORT")
            .expect("$PORT is not set in `.env` file")
            .parse()
            .unwrap()
    });

    let user = cli
        .user
        .clone()
        .unwrap_or_else(|| std::env::var("USER").expect("$USER is not set in `.env` file"));

    let public_key_path = cli.public_key_path.clone().unwrap_or_else(|| {
        std::env::var("PUBLIC_KEY_PATH").expect("$PUBLIC_KEY_PATH is not set in `.env` file")
    });

    let private_key_path = cli.private_key_path.clone().unwrap_or_else(|| {
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

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_get_server_connection_details_from_env() {
        env::set_var("HOST", "192.168.178.255");
        env::set_var("PORT", "22");
        env::set_var("USER", "pi");
        env::set_var("PUBLIC_KEY_PATH", "N/A");
        env::set_var("PRIVATE_KEY_PATH", "N/A");

        assert_eq!(env::var("HOST").unwrap(), "192.168.178.255");

        let cli = Cli {
            command: crate::cli::Command::Test,
            host: None,
            port: None,
            user: None,
            public_key_path: None,
            private_key_path: None,
        };

        let server = get_server_information(&cli);

        assert_eq!(server.host, "192.168.178.255");
        assert_eq!(server.port, 22);
        assert_eq!(server.user, "pi");
        assert_eq!(server.keys.public_key_path, "N/A");
        assert_eq!(server.keys.private_key_path, "N/A");
    }

    #[test]
    fn test_get_server_connection_details_from_cli() {
        env::set_var("HOST", "XXX");
        env::set_var("PORT", "XXX");
        env::set_var("USER", "XXX");
        env::set_var("PUBLIC_KEY_PATH", "N/A");
        env::set_var("PRIVATE_KEY_PATH", "N/A");

        let cli = Cli {
            command: crate::cli::Command::Test,
            host: Some("192.168.178.255".to_string()),
            port: Some(22),
            user: Some("pi".to_string()),
            public_key_path: None,
            private_key_path: None,
        };

        let server = get_server_information(&cli);

        assert_eq!(server.host, "192.168.178.255");
        assert_eq!(server.port, 22);
        assert_eq!(server.user, "pi");
        assert_eq!(server.keys.public_key_path, "N/A");
        assert_eq!(server.keys.private_key_path, "N/A");
    }
}
