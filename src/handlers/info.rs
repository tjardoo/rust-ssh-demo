use crate::{
    commands::info::InfoCommand,
    utils::{Location, ServerCommand},
};

pub fn handle(command: &InfoCommand) -> ServerCommand {
    let command = match command {
        InfoCommand::Hardware => "cat /proc/version".to_string(),
        InfoCommand::Memory => "cat /proc/meminfo".to_string(),
        InfoCommand::Cpu => "cat /proc/cpuinfo".to_string(),
        InfoCommand::Partitions => "cat /proc/partitions".to_string(),
        InfoCommand::Temperature => "vcgencmd measure_temp".to_string(),
        InfoCommand::Uptime => "uptime".to_string(),
        InfoCommand::Version => "uname -a".to_string(),
        InfoCommand::Pwd => "pwd".to_string(),
        InfoCommand::Du { directory } => format!("du -h --max-depth=1 {}", directory),
        InfoCommand::ServiceInstalledCheck { service } => {
            format!(r#"dpkg-query -W -f='${{Status}}' {}"#, service)
        }
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
        let server_command = handle(&InfoCommand::Hardware);

        assert_eq!(server_command.command, "cat /proc/version");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_temperature() {
        let server_command = handle(&InfoCommand::Temperature);

        assert_eq!(server_command.command, "vcgencmd measure_temp");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_pwd() {
        let server_command = handle(&InfoCommand::Pwd);

        assert_eq!(server_command.command, "pwd");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_du() {
        let server_command = handle(&InfoCommand::Du {
            directory: "/var/log".to_string(),
        });

        assert_eq!(server_command.command, "du -h --max-depth=1 /var/log");
        assert_eq!(server_command.location, Location::Remote);
    }

    #[test]
    fn test_handle_service_installed_check() {
        let server_command = handle(&&InfoCommand::ServiceInstalledCheck {
            service: "nginx".to_string(),
        });

        assert_eq!(server_command.command, "dpkg-query -W -f='${Status}' nginx");
        assert_eq!(server_command.location, Location::Remote);
    }
}
