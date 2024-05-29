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
    } else {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(server_command.command.as_str())
            .output()
    };

    match output {
        Ok(output) => {
            if output.status.success() {
                print_exit_status(
                    output.status.code().unwrap(),
                    Some(String::from_utf8_lossy(&output.stdout).to_string()),
                );
            } else {
                print_exit_status(
                    output.status.code().unwrap(),
                    Some(String::from_utf8_lossy(&output.stderr).to_string()),
                );
            }
        }
        Err(error) => {
            print_exit_status(1, Some(error.to_string()));
        }
    }
}

pub fn run_remote_command(server: Server, server_command: ServerCommand) {
    let session = ssh::create_ssh_session(server);

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

    match channel.wait_close() {
        Ok(_) => print_exit_status(channel.exit_status().unwrap(), None),
        Err(error) => print_exit_status(
            channel.exit_status().unwrap(),
            Some(error.message().to_string()),
        ),
    }
}

fn print_command(command: &str) {
    println!("{} {}", "Executing:".black(), command.black(),);
}

fn print_exit_status(exit_status: i32, message: Option<String>) {
    if exit_status == 0 {
        println!(
            "{} {}",
            "Exit status:".black(),
            exit_status.to_string().black(),
        );
    } else {
        println!(
            "{} {}",
            "Exit status:".black(),
            exit_status.to_string().red(),
        );
    }

    if let Some(message) = message {
        if message.is_empty() {
            return;
        }

        println!("{}", message.red().italic());
    }
}
