use crate::utils::{args::SharedServerArgs, executer::ServerCommand};

pub fn handle(args: SharedServerArgs) -> ServerCommand {
    ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        server_args: args,
    }
}
