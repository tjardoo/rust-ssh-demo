use crate::{
    commands::info::InfoCommand,
    utils::{Location, ServerCommand},
};

pub fn handle(command: InfoCommand) -> ServerCommand {
    let command = match command {
        InfoCommand::Hardware => "cat /proc/version".to_string(),
        InfoCommand::Memory => "cat /proc/meminfo".to_string(),
        InfoCommand::Cpu => "cat /proc/cpuinfo".to_string(),
        InfoCommand::Partitions => "cat /proc/partitions".to_string(),
        InfoCommand::Temperature => "vcgencmd measure_temp".to_string(),
        InfoCommand::Uptime => "uptime".to_string(),
        InfoCommand::Version => "uname -a".to_string(),
        InfoCommand::Pwd => "pwd".to_string(),
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
    fn test_handle_hardware() {
        let server_command = handle(InfoCommand::Hardware);

        assert_eq!(server_command.command, "cat /proc/version");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_temperature() {
        let server_command = handle(InfoCommand::Temperature);

        assert_eq!(server_command.command, "vcgencmd measure_temp");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_pwd() {
        let server_command = handle(InfoCommand::Pwd);

        assert_eq!(server_command.command, "pwd");
        assert_eq!(server_command.location, Location::Remote);
    }
}
