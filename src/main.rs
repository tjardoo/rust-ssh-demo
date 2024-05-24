use clap::Parser;
use dotenv::dotenv;
use std::io::prelude::*;

use crate::cli::{Cli, Command};

mod cli;
mod commands;
mod handlers;
mod ssh;
mod utils;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let server_command = match cli.command {
        Command::Test(args) => handlers::test::handle(args),
        Command::Info(command) => handlers::info::handle(command),
        Command::Action(command) => handlers::action::handle(command),
        Command::File(command) => handlers::file::handle(command),
    };

    let session = ssh::get_ssh_session(server_command.server_args);

    let mut channel = session.channel_session().unwrap();
    channel.exec(&server_command.command).unwrap();

    let mut result = String::new();
    channel.read_to_string(&mut result).unwrap();

    println!("{}", result);

    ssh::close_channel(channel);
}
