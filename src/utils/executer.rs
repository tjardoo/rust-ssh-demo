use super::args::SharedServerArgs;

pub struct ServerCommand {
    pub command: String,
    pub server_args: SharedServerArgs,
}
