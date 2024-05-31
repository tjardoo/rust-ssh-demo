use crate::{commands::control::ControlCommand, utils::ServerCommand};

pub fn handle(command: &ControlCommand) -> Vec<ServerCommand> {
    let command = match command {
        ControlCommand::Update => "sudo apt-get update && sudo apt-get -y upgrade".to_string(),
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
    fn test_handle_update() {
        let server_commands = handle(&ControlCommand::Update);

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "sudo apt-get update && sudo apt-get -y upgrade"
        );
    }
}
