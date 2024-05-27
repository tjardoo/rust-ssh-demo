use crate::{
    commands::file::FileCommand,
    utils::{
        args::get_server_connection_detail,
        executer::{Location, ServerCommand},
    },
};

pub fn handle(command: FileCommand) -> ServerCommand {
    let (command, server_args) = match command {
        FileCommand::Upload {
            file,
            destination,
            server_args,
        } => {
            let server = get_server_connection_detail(server_args.clone());

            let command = format!(
                "scp {} {}@{}:{}",
                file, server.user, server.host, destination
            );

            (command, server_args)
        }
        FileCommand::Download {
            file,
            destination,
            server_args,
        } => {
            let server = get_server_connection_detail(server_args.clone());

            let command = format!(
                "scp {}@{}:{} {}",
                server.user, server.host, file, destination
            );

            (command, server_args)
        }
    };

    ServerCommand {
        command,
        server_args,
        location: Location::Local,
    }
}
