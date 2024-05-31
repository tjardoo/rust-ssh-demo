use crate::{commands::info::InfoCommand, utils::ServerCommand};

pub fn handle(command: &InfoCommand) -> Vec<ServerCommand> {
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

    vec![ServerCommand {
        command,
        print_output: true,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_hardware() {
        let server_commands = handle(&InfoCommand::Hardware);

        assert_eq!(server_commands.len(), 1);
        assert_eq!(server_commands.get(0).unwrap().command, "cat /proc/version");
    }

    #[test]
    fn test_handle_temperature() {
        let server_commands = handle(&InfoCommand::Temperature);

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "vcgencmd measure_temp"
        );
    }

    #[test]
    fn test_handle_pwd() {
        let server_commands = handle(&InfoCommand::Pwd);

        assert_eq!(server_commands.len(), 1);
        assert_eq!(server_commands.get(0).unwrap().command, "pwd");
    }

    #[test]
    fn test_handle_du() {
        let server_commands = handle(&InfoCommand::Du {
            directory: "/var/log".to_string(),
        });

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "du -h --max-depth=1 /var/log"
        );
    }

    #[test]
    fn test_handle_service_installed_check() {
        let server_commands = handle(&&InfoCommand::ServiceInstalledCheck {
            service: "nginx".to_string(),
        });

        assert_eq!(server_commands.len(), 1);
        assert_eq!(
            server_commands.get(0).unwrap().command,
            "dpkg-query -W -f='${Status}' nginx"
        );
    }
}
