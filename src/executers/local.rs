use crate::{
    helpers::{print_command, print_exit_status},
    utils::{Server, ServerCommand},
};

pub fn run(_server: Server, server_command: ServerCommand) {
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
