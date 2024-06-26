use crate::{
    commands::file::FileCommand,
    utils::{Location, Server, ServerCommand},
};

pub fn handle(server: Server, command: &FileCommand) -> ServerCommand {
    let (command, location) = match command {
        FileCommand::Upload { file, destination } => (
            format!(
                "scp -P {} {} {}@{}:{}",
                server.port, file, server.user, server.host, destination
            ),
            Location::Local,
        ),
        FileCommand::Download { file, destination } => (
            format!(
                "scp -P {} {}@{}:{} {}",
                server.port, server.user, server.host, file, destination
            ),
            Location::Local,
        ),
        FileCommand::View { file } => (format!("cat {}", file), Location::Remote),
    };

    ServerCommand { command, location }
}

#[cfg(test)]
mod tests {
    use crate::utils::Keys;

    use super::*;

    #[test]
    fn test_handle_upload() {
        let server = Server {
            host: "192.168.178.255".to_string(),
            port: 22,
            user: "pi".to_string(),
            keys: Keys {
                public_key_path: "N/A".to_string(),
                private_key_path: "N/A".to_string(),
            },
        };

        let server_command = handle(
            server,
            &FileCommand::Upload {
                file: "test.txt".to_string(),
                destination: "/home/pi".to_string(),
            },
        );

        assert_eq!(
            server_command.command,
            "scp -P 22 test.txt pi@192.168.178.255:/home/pi"
        );
        assert_eq!(server_command.location, Location::Local);
    }

    #[test]
    fn test_handle_download() {
        let server = Server {
            host: "192.168.178.255".to_string(),
            port: 22,
            user: "pi".to_string(),
            keys: Keys {
                public_key_path: "N/A".to_string(),
                private_key_path: "N/A".to_string(),
            },
        };

        let server_command = handle(
            server,
            &FileCommand::Download {
                file: "/home/pi/test.txt".to_string(),
                destination: ".".to_string(),
            },
        );

        assert_eq!(
            server_command.command,
            "scp -P 22 pi@192.168.178.255:/home/pi/test.txt ."
        );
        assert_eq!(server_command.location, Location::Local);
    }

    #[test]
    fn test_handle_view() {
        let server = Server {
            host: "192.168.178.255".to_string(),
            port: 22,
            user: "pi".to_string(),
            keys: Keys {
                public_key_path: "N/A".to_string(),
                private_key_path: "N/A".to_string(),
            },
        };

        let server_command = handle(
            server,
            &FileCommand::View {
                file: "/var/log/auth.log".to_string(),
            },
        );

        assert_eq!(server_command.command, "cat /var/log/auth.log");
        assert_eq!(server_command.location, Location::Remote);
    }
}
