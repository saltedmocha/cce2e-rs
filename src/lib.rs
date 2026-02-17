use std::{net, process};

pub struct ListenerOption {}

impl ListenerOption {}

pub fn start_listening() -> net::TcpListener {
    let listener: net::TcpListener =
        net::TcpListener::bind("localhost:7007").unwrap_or_else(|err| {
            println!("Failed to start server: {}, exiting...", err);
            process::exit(1);
        });

    listener
}
