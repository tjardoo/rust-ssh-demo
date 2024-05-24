use crate::{commands::info::InfoCommand, utils::executer::ServerCommand};

pub fn handle(command: InfoCommand) -> ServerCommand {
    match command {
        InfoCommand::Hardware(args) => ServerCommand {
            command: "cat /proc/version".to_string(),
            server_args: args,
        },
        InfoCommand::Memory(args) => ServerCommand {
            command: "cat /proc/meminfo".to_string(),
            server_args: args,
        },
        InfoCommand::Partitions(args) => ServerCommand {
            command: "cat /proc/partitions".to_string(),
            server_args: args,
        },
        InfoCommand::Temperature(args) => ServerCommand {
            command: "vcgencmd measure_temp".to_string(),
            server_args: args,
        },
    }
}
