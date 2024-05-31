use clap::Parser;
use dotenv::dotenv;
use utils::get_server_information;

use crate::cli::{Cli, Command};

mod cli;
mod commands;
mod formatter;
mod formatters;
mod handlers;
mod helpers;
mod remote;
mod ssh;
mod utils;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

type OptionalFormatter = Option<Box<dyn Fn(&str) -> String>>;

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let server = get_server_information(&cli);

    let commands = match &cli.command {
        Command::Test => handlers::test::handle(),
        Command::Info(command) => handlers::info::handle(command),
        Command::Action(command) => handlers::action::handle(command),
        Command::File(command) => handlers::file::handle(command),
        Command::Control(command) => handlers::control::handle(command),
        Command::Install(command) => handlers::install::handle(command),
    };

    remote::run(server, commands, &cli.command);
}
