use crate::{commands::install::InstallCommand, utils::ServerCommand};

pub fn handle(command: &InstallCommand) -> Vec<ServerCommand> {
    let commands = match command {
        InstallCommand::Nginx => [
            ("sudo apt install nginx -y".to_string(), false),
            ("dpkg-query -W -f='${Status}' nginx".to_string(), true),
        ],
    };

    commands
        .iter()
        .map(|(command, print_output)| ServerCommand {
            command: command.to_string(),
            print_output: *print_output,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_nginx() {
        let server_commands = handle(&InstallCommand::Nginx);

        assert_eq!(server_commands.len(), 2);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "sudo apt install nginx -y"
        );
        assert_eq!(
            server_commands.get(1).unwrap().command,
            "dpkg-query -W -f='${Status}' nginx"
        );
    }
}
