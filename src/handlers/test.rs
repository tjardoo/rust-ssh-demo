use crate::exec::{Location, ServerCommand};

pub fn handle() -> ServerCommand {
    ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        location: Location::Remote,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let server_command = handle();

        assert_eq!(server_command.command, "echo 'Hello, World!'");
        assert_eq!(server_command.location, Location::Remote);
    }
}
