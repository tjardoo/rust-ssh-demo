use std::io::prelude::*;

use crate::{
    helpers::{print_command, print_exit_status},
    ssh,
    utils::{Server, ServerCommand},
};

pub fn run(server: Server, server_command: ServerCommand) {
    let session = ssh::create_ssh_session(server);

    print_command(server_command.command.as_str());

    let mut channel = session.channel_session().unwrap();
    channel.exec(&server_command.command).unwrap();

    let mut error_result = String::new();
    channel.stderr().read_to_string(&mut error_result).unwrap();

    if !error_result.is_empty() {
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
