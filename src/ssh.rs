use colored::Colorize;
use ssh2::{Channel, Session};
use std::{net::TcpStream, path::Path};

use crate::utils::args::{get_server_connection_detail, SharedServerArgs};

pub fn get_ssh_session(args: SharedServerArgs) -> Session {
    let server = get_server_connection_detail(args);

    println!(
        "{} {}{}{}{}{}",
        "Connecting to:".black(),
        server.user.black(),
        "@".black(),
        server.host.black(),
        ":".black(),
        server.port.black()
    );

    let tcp_stream = TcpStream::connect(format!("{}:{}", server.host, server.port));

    let tcp_stream = match tcp_stream {
        Ok(tcp_stream) => tcp_stream,
        Err(_) => {
            println!(
                "{} {}",
                "Failed to connect to:".black(),
                server.host.white().on_red()
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
        .userauth_pubkey_file(&server.user, Some(public_key_path), private_key_path, None)
        .unwrap();

    session
}

pub fn close_channel(mut channel: Channel) {
    channel.wait_close().unwrap();

    match channel.exit_status() {
        Ok(status) => {
            if status == 1 {
                println!(
                    "{} {}",
                    "Exit status:".black(),
                    channel
                        .exit_status()
                        .unwrap()
                        .to_string()
                        .white()
                        .on_bright_red()
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
