use colored::Colorize;
use ssh2::Session;
use std::{net::TcpStream, path::Path};

use crate::utils::Server;

pub fn create_ssh_session(server: Server) -> Session {
    println!(
        "{} {}",
        "Connecting to:".black(),
        server.to_string().black()
    );

    let tcp_stream = TcpStream::connect(format!("{}:{}", server.host, server.port));

    let tcp_stream = match tcp_stream {
        Ok(tcp_stream) => tcp_stream,
        Err(_) => {
            println!(
                "{} {}",
                "Failed to connect to:".black(),
                server.to_string().white().on_red()
            );

            std::process::exit(1);
        }
    };

    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp_stream);
    session.handshake().unwrap();

    let public_key_path = Path::new(server.keys.public_key_path.as_str());
    assert!(public_key_path.exists());

    let private_key_path = Path::new(server.keys.private_key_path.as_str());
    assert!(private_key_path.exists());

    session
        .userauth_pubkey_file(&server.user, Some(public_key_path), private_key_path, None)
        .unwrap();

    session
}
