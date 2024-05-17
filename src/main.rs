use clap::Parser;
use cli::SharedServerArgs;
use dotenv::dotenv;
use std::io::prelude::*;

use crate::cli::{Cli, Command};

mod cli;
mod handler;
mod ssh;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct ServerCommand {
    pub command: String,
    pub server_args: SharedServerArgs,
}

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let server_command = match cli.command {
        Command::Test(args) => handler::handle_test_command(args),
        Command::Info(command) => handler::handle_info_command(command),
        Command::Action(command) => handler::handle_action_command(command),
    };

    let session = ssh::get_ssh_session(server_command.server_args);

    let mut channel = session.channel_session().unwrap();
    channel.exec(&server_command.command).unwrap();

    let mut result = String::new();
    channel.read_to_string(&mut result).unwrap();

    println!("{}", result);

    ssh::close_channel(channel);
}
