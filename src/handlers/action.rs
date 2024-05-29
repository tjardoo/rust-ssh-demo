use crate::{
    commands::action::ActionCommand,
    utils::{Location, ServerCommand},
};

pub fn handle(command: ActionCommand) -> ServerCommand {
    let command = match command {
        ActionCommand::Reboot => "sudo reboot".to_string(),
        ActionCommand::Shutdown => "sudo shutdown -h now".to_string(),
        ActionCommand::Cat { file } => format!("cat {}", file),
    };

    ServerCommand {
        command,
        location: Location::Remote,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_reboot() {
        let server_command = handle(ActionCommand::Reboot);

        assert_eq!(server_command.command, "sudo reboot");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_shutdown() {
        let server_command = handle(ActionCommand::Shutdown);

        assert_eq!(server_command.command, "sudo shutdown -h now");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_cat() {
        let server_command = handle(ActionCommand::Cat {
            file: "/var/log/auth.log".to_string(),
        });

        assert_eq!(server_command.command, "cat /var/log/auth.log");
        assert_eq!(server_command.location, Location::Remote);
    }
}
