use crate::{
    cli::{ActionCommand, InfoCommand},
    ServerCommand, SharedServerArgs,
};

pub fn handle_test_command(args: SharedServerArgs) -> ServerCommand {
    ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        server_args: args,
    }
}

pub fn handle_info_command(command: InfoCommand) -> ServerCommand {
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

pub fn handle_action_command(command: ActionCommand) -> ServerCommand {
    match command {
        ActionCommand::Reboot(args) => ServerCommand {
            command: "sudo reboot".to_string(),
            server_args: args,
        },
        ActionCommand::Shutdown(args) => ServerCommand {
            command: "sudo shutdown -h now".to_string(),
            server_args: args,
        },
    }
}
