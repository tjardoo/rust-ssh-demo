use crate::{
    commands::file::FileCommand,
    utils::{
        args::Server,
        executer::{Location, ServerCommand},
    },
};

pub fn handle(server: Server, command: FileCommand) -> ServerCommand {
    let command = match command {
        FileCommand::Upload { file, destination } => {
            format!(
                "scp {} {}@{}:{}",
                file, server.user, server.host, destination
            )
        }
        FileCommand::Download { file, destination } => {
            format!(
                "scp {}@{}:{} {}",
                server.user, server.host, file, destination
            )
        }
    };

    ServerCommand {
        command,
        location: Location::Local,
    }
}
