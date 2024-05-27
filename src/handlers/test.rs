use crate::utils::executer::{Location, ServerCommand};

pub fn handle() -> ServerCommand {
    ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        location: Location::Remote,
    }
}
