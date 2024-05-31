use crate::{
    commands::install::InstallCommand,
    utils::{Location, ServerCommand},
};

pub fn handle(command: &InstallCommand) -> ServerCommand {
    let command = match command {
        InstallCommand::Nginx => "sudo apt install nginx -y".to_string(),
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
    fn test_handle_nginx() {
        let server_command = handle(&InstallCommand::Nginx);

        assert_eq!(server_command.command, "sudo apt install nginx -y");
        assert_eq!(server_command.location, Location::Remote);
    }
}
