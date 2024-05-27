use crate::{
    commands::action::ActionCommand,
    utils::executer::{Location, ServerCommand},
};

pub fn handle(command: ActionCommand) -> ServerCommand {
    let command = match command {
        ActionCommand::Reboot => "sudo reboot".to_string(),
        ActionCommand::Shutdown => "sudo shutdown -h now".to_string(),
    };

    ServerCommand {
        command,
        location: Location::Remote,
    }
}
