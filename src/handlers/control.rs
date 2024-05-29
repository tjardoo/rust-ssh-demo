use crate::{
    commands::control::ControlCommand,
    utils::{Location, ServerCommand},
};

pub fn handle(command: ControlCommand) -> ServerCommand {
    let command = match command {
        ControlCommand::Update => "sudo apt-get update && sudo apt-get -y upgrade".to_string(),
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
    fn test_handle_update() {
        let server_command = handle(ControlCommand::Update);

        assert_eq!(
            server_command.command,
            "sudo apt-get update && sudo apt-get -y upgrade"
        );
        assert_eq!(server_command.location, Location::Remote);
    }
}
