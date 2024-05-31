use crate::{commands::file::FileCommand, utils::ServerCommand};

pub fn handle(command: &FileCommand) -> Vec<ServerCommand> {
    let command = match command {
        FileCommand::View { file } => format!("cat {}", file),
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
    fn test_handle_view() {
        let server_commands = handle(&FileCommand::View {
            file: "/var/log/auth.log".to_string(),
        });

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "cat /var/log/auth.log"
        );
    }
}
