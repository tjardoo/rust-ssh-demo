use super::args::SharedServerArgs;

pub struct ServerCommand {
    pub command: String,
    pub server_args: SharedServerArgs,
    pub location: Location,
}

pub enum Location {
    Local,
    Remote,
}
