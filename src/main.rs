use clap::Parser;
use dotenv::dotenv;
use exec::{run_local_command, run_remote_command, Location};
use utils::get_server_information;

use crate::cli::{Cli, Command};

mod cli;
mod commands;
mod exec;
mod handlers;
mod ssh;
mod utils;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let server = get_server_information(&cli);

    let server_command = match cli.command {
        Command::Test => handlers::test::handle(),
        Command::Info(command) => handlers::info::handle(command),
        Command::Action(command) => handlers::action::handle(command),
        Command::File(command) => handlers::file::handle(server.clone(), command),
    };

    match server_command.location {
        Location::Local => run_local_command(server, server_command),
        Location::Remote => run_remote_command(server, server_command),
    }
}
