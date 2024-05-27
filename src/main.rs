use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use std::io::prelude::*;
use utils::{
    args::{get_server_connection_detail, Server},
    executer::{Location, ServerCommand},
};

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

    let server = get_server_connection_detail(
        cli.host,
        cli.port,
        cli.user,
        cli.public_key_path,
        cli.private_key_path,
    );

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

fn run_local_command(_server: Server, server_command: ServerCommand) {
    print_command(server_command.command.as_str());

    let output = if cfg!(target_os = "windows") {
        std::process::Command::new("cmd")
            .args(["/C", server_command.command.as_str()])
            .output()
            .expect("failed to execute process");
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(server_command.command.as_str())
            .output()
            .expect("failed to execute process");
    };

    println!("{:?}", output);
}

fn run_remote_command(server: Server, server_command: ServerCommand) {
    let session = ssh::get_ssh_session(server);

    print_command(server_command.command.as_str());

    let mut channel = session.channel_session().unwrap();
    channel.exec(&server_command.command).unwrap();

    let mut error_result = String::new();
    channel.stderr().read_to_string(&mut error_result).unwrap();

    if error_result.is_empty() == false {
        println!("{}", error_result);
    }

    let mut result = String::new();
    channel.read_to_string(&mut result).unwrap();

    println!("{}", result);

    ssh::close_channel(channel);
}

fn print_command(command: &str) {
    println!("{} {}", "Executing:".black(), command.black(),);
}
