use std::{
    io::{Read, Write},
    net, process,
};

pub fn start_listening() -> net::TcpListener {
    let listener: net::TcpListener =
        net::TcpListener::bind("localhost:7007").unwrap_or_else(|err| {
            println!("Failed to start server: {}, exiting...", err);
            process::exit(1);
        });

    if let Err(err) = listener.set_ttl(128) {
        println!("Failed to set TTL: {}, using default...", err);
    };

    listener
}

pub fn handle_client(stream: &mut net::TcpStream) {
    let mut buf: [u8; 2048] = [0; 2048];
    let Ok(addr) = stream.peer_addr() else {
        println!("Failed to receive connection");
        let _ = stream.shutdown(net::Shutdown::Both);
        return;
    };

    println!("Received connection from {}", addr);

    loop {
        let Ok(stream_read) = stream.read(&mut buf) else {
            println!("Failed to read input data");
            return;
        };

        if stream_read == 0 {
            println!("Connection {} closed/disconnected", addr);
            return;
        }

        if let Err(err) = stream.write_all(&buf[..stream_read]) {
            println!("Failed to write data, {}", err);
            return;
        };

        buf = [0; 2048];
    }
}

pub mod args;
pub mod network;
