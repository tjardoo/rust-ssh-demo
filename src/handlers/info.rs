use crate::{
    commands::info::InfoCommand,
    utils::executer::{Location, ServerCommand},
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
        InfoCommand::CurrentDir => "pwd".to_string(),
    };

    ServerCommand {
        command,
        location: Location::Remote,
    }
}
