use crate::{commands::action::ActionCommand, utils::ServerCommand};

pub fn handle(command: &ActionCommand) -> Vec<ServerCommand> {
    let command = match command {
        ActionCommand::Reboot => "sudo reboot".to_string(),
        ActionCommand::Shutdown => "sudo shutdown -h now".to_string(),
    };

    vec![ServerCommand {
        command,
        print_output: true,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_reboot() {
        let server_commands = handle(&ActionCommand::Reboot);

        assert_eq!(server_commands.len(), 1);
        assert_eq!(server_commands.get(0).unwrap().command, "sudo reboot");
    }

    #[test]
    fn test_handle_shutdown() {
        let server_commands = handle(&ActionCommand::Shutdown);

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "sudo shutdown -h now"
        );
    }
}
