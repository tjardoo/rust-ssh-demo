use clap::{Parser, Subcommand};
use colored::Colorize;
use dotenv::dotenv;
use ssh2::{Channel, Session};
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::Path;

const _VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Test connection
    Test(SharedServerArgs),
    #[clap(subcommand)]
    /// Get system information
    Info(InfoCommand),
    /// Perform a system action
    #[clap(subcommand)]
    Action(ActionCommand),
}

#[derive(Subcommand)]
pub enum InfoCommand {
    /// Get hardware information
    Hardware(SharedServerArgs),
    /// Get memory information
    Memory(SharedServerArgs),
    /// Get partition information
    Partitions(SharedServerArgs),
    /// Get temperature information
    Temperature(SharedServerArgs),
}

#[derive(Subcommand)]
pub enum ActionCommand {
    /// Reboot the system
    Reboot(SharedServerArgs),
    /// Shutdown the system
    Shutdown(SharedServerArgs),
}

#[derive(Parser)]
pub struct SharedServerArgs {
    /// Optional hostname or IP address
    #[clap(long = "host")]
    pub host: Option<String>,
    /// Optional port number
    #[clap(long = "port")]
    pub port: Option<String>,
    /// Optional username
    #[clap(long = "user")]
    pub user: Option<String>,
}

pub struct ServerCommand {
    pub command: String,
    pub server_args: SharedServerArgs,
}

fn main() {
    dotenv().ok();

    let cli = Cli::parse();

    let server_command = match cli.command {
        Command::Test(args) => handle_test_command(args),
        Command::Info(command) => handle_info_command(command),
        Command::Action(command) => handle_action_command(command),
    };

    let session = get_ssh_session(server_command.server_args);

    let mut channel = session.channel_session().unwrap();
    channel.exec(&server_command.command).unwrap();

    let mut result = String::new();
    channel.read_to_string(&mut result).unwrap();

    println!("{}", result);

    close_channel(channel);
}

fn handle_test_command(args: SharedServerArgs) -> ServerCommand {
    ServerCommand {
        command: "echo 'Hello, World!'".to_string(),
        server_args: args,
    }
}

fn handle_info_command(command: InfoCommand) -> ServerCommand {
    match command {
        InfoCommand::Hardware(args) => ServerCommand {
            command: "cat /proc/version".to_string(),
            server_args: args,
        },
        InfoCommand::Memory(args) => ServerCommand {
            command: "cat /proc/meminfo".to_string(),
            server_args: args,
        },
        InfoCommand::Partitions(args) => ServerCommand {
            command: "cat /proc/partitions".to_string(),
            server_args: args,
        },
        InfoCommand::Temperature(args) => ServerCommand {
            command: "vcgencmd measure_temp".to_string(),
            server_args: args,
        },
    }
}

fn handle_action_command(command: ActionCommand) -> ServerCommand {
    match command {
        ActionCommand::Reboot(args) => ServerCommand {
            command: "sudo reboot".to_string(),
            server_args: args,
        },
        ActionCommand::Shutdown(args) => ServerCommand {
            command: "sudo shutdown -h now".to_string(),
            server_args: args,
        },
    }
}

fn get_ssh_session(args: SharedServerArgs) -> Session {
    let host = args
        .host
        .unwrap_or_else(|| std::env::var("HOST").expect("$HOST is not set in `.env` file"));

    let port = args
        .port
        .unwrap_or_else(|| std::env::var("PORT").expect("$PORT is not set in `.env` file"));

    let user = args
        .user
        .unwrap_or_else(|| std::env::var("USER").expect("$USER is not set in `.env` file"));

    println!(
        "{} {}{}{}{}{}",
        "Connecting to:".black(),
        user.black(),
        "@".black(),
        host.black(),
        ":".black(),
        port.black()
    );

    let tcp_stream = TcpStream::connect(format!("{}:{}", host, port));

    let tcp_stream = match tcp_stream {
        Ok(tcp_stream) => tcp_stream,
        Err(_) => {
            println!(
                "{} {}",
                "Failed to connect to:".black(),
                host.white().on_red()
            );

            std::process::exit(1);
        }
    };

    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp_stream);
    session.handshake().unwrap();

    let public_key_path = Path::new(".ssh/id_rsa.pub");
    assert!(public_key_path.exists());

    let private_key_path = Path::new(".ssh/id_rsa");
    assert!(private_key_path.exists());

    session
        .userauth_pubkey_file(&user, Some(public_key_path), private_key_path, None)
        .unwrap();

    session
}

fn close_channel(mut channel: Channel) {
    channel.wait_close().unwrap();

    match channel.exit_status() {
        Ok(status) => {
            if status == 0 {
                println!(
                    "{} {}",
                    "Exit status:".black(),
                    channel.exit_status().unwrap().to_string().black()
                );
            } else {
                println!(
                    "{} {}",
                    "Exit status:".black(),
                    channel.exit_status().unwrap().to_string().black()
                );
            }
        }
        Err(_) => {
            println!(
                "{} {}",
                "Exit status:".black(),
                "Failed".white().on_bright_red()
            );
        }
    }
}
