use colored::Colorize;
use ssh2::{Channel, Session};
use std::{net::TcpStream, path::Path};

use crate::utils::args::SharedServerArgs;

pub fn get_ssh_session(args: SharedServerArgs) -> Session {
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

pub fn close_channel(mut channel: Channel) {
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
