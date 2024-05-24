use crate::{commands::file::FileCommand, utils::executer::ServerCommand};

pub fn handle(command: FileCommand) -> ServerCommand {
    match command {
        FileCommand::Upload(args) => ServerCommand {
            command: "".to_string(),
            server_args: args,
        },
        FileCommand::Download(args) => ServerCommand {
            command: "".to_string(),
            server_args: args,
        },
    }
}
