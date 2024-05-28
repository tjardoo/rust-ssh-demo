use colored::Colorize;
use std::io::prelude::*;

use crate::{ssh, utils::Server};

pub struct ServerCommand {
    pub command: String,
    pub location: Location,
}

#[derive(Debug, PartialEq)]
pub enum Location {
    Local,
    Remote,
}

pub fn run_local_command(_server: Server, server_command: ServerCommand) {
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

pub fn run_remote_command(server: Server, server_command: ServerCommand) {
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
