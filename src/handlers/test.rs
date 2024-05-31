use crate::utils::ServerCommand;

pub fn handle() -> Vec<ServerCommand> {
    vec![ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        print_output: true,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle() {
        let server_commands = handle();

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "echo 'Hello, World!'"
        );
    }
}
