use crate::{
    commands::action::ActionCommand,
    utils::executer::{Location, ServerCommand},
};

pub fn handle(command: ActionCommand) -> ServerCommand {
    let (command, server_args) = match command {
        ActionCommand::Reboot(args) => ("sudo reboot".to_string(), args),
        ActionCommand::Shutdown(args) => ("sudo shutdown -h now".to_string(), args),
    };

    ServerCommand {
        command,
        server_args,
        location: Location::Remote,
    }
}
