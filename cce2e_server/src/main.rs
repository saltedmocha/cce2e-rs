use std::net;

use clap::Parser;

fn main() {
    cce2e_rs::args::AppArgs::parse();

    let listener: net::TcpListener = cce2e_rs::start_listening();

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream_info) => cce2e_rs::handle_client(&mut stream_info),
            Err(err) => {
                println!("Failed to connect, {}", err);
                return;
            }
        }
    }
}
