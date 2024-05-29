use clap::Parser;
use dotenv::dotenv;
use utils::{get_server_information, Location};

use crate::cli::{Cli, Command};

mod cli;
mod commands;
mod executers;
mod handlers;
mod helpers;
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
        Command::Control(command) => handlers::control::handle(command),
    };

    match server_command.location {
        Location::Local => executers::local::run(server, server_command),
        Location::Remote => executers::remote::run(server, server_command),
    }
}
