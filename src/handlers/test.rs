use crate::utils::{
    args::SharedServerArgs,
    executer::{Location, ServerCommand},
};

pub fn handle(args: SharedServerArgs) -> ServerCommand {
    ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        server_args: args,
        location: Location::Remote,
    }
}
