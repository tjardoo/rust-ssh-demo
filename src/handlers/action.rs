use crate::{commands::action::ActionCommand, utils::executer::ServerCommand};

pub fn handle(command: ActionCommand) -> ServerCommand {
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
