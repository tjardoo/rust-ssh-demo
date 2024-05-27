use crate::{
    commands::info::InfoCommand,
    utils::executer::{Location, ServerCommand},
};

pub fn handle(command: InfoCommand) -> ServerCommand {
    let (command, server_args) = match command {
        InfoCommand::Hardware(args) => ("cat /proc/version".to_string(), args),
        InfoCommand::Memory(args) => ("cat /proc/meminfo".to_string(), args),
        InfoCommand::Partitions(args) => ("cat /proc/partitions".to_string(), args),
        InfoCommand::Temperature(args) => ("vcgencmd measure_temp".to_string(), args),
    };

    ServerCommand {
        command,
        server_args,
        location: Location::Remote,
    }
}
