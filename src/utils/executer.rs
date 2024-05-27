pub struct ServerCommand {
    pub command: String,
    pub location: Location,
}

pub enum Location {
    Local,
    Remote,
}
