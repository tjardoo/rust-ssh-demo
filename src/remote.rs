use std::io::prelude::*;

use ssh2::Channel;

use crate::{
    cli::Command,
    formatter::find_formatter,
    helpers::{print_command, print_exit_status},
    ssh,
    utils::{Server, ServerCommand},
};

pub fn run(server: Server, server_command: Vec<ServerCommand>, command: &Command) {
    let session = ssh::create_ssh_session(server);

    for server_command in server_command {
        let mut channel = session.channel_session().unwrap();

        run_command(&mut channel, server_command, command);
    }
}

fn run_command(channel: &mut Channel, server_command: ServerCommand, command: &Command) {
    print_command(server_command.command.as_str());

    let formatter = find_formatter(command);

    channel.exec(&server_command.command).unwrap();

    let mut error_result = String::new();
    channel.stderr().read_to_string(&mut error_result).unwrap();

    if !error_result.is_empty() && formatter.is_none() {
        println!("{}", error_result);
    }

    let result = read_buffer(channel, formatter.is_some(), server_command.print_output);

    if let Some(formatter) = formatter {
        println!("{}", formatter(&result));
    }

    match channel.wait_close() {
        Ok(_) => print_exit_status(channel.exit_status().unwrap(), None),
        Err(error) => print_exit_status(
            channel.exit_status().unwrap(),
            Some(error.message().to_string()),
        ),
    }
}

fn read_buffer(channel: &mut Channel, is_formatter: bool, is_print_ouput: bool) -> String {
    let mut buffer = [0; 1024];
    let mut result = String::new();

    loop {
        match channel.read(&mut buffer) {
            Ok(n) => {
                if n == 0 {
                    break;
                }

                let output = String::from_utf8_lossy(&buffer[..n]);

                if is_formatter {
                    result.push_str(&output);
                } else if is_print_ouput {
                    print!("{}", output);
                }
            }
            Err(error) => {
                println!("{}", error);

                break;
            }
        }
    }

    result
}
