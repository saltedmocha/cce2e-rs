use std::{
    io::{Read, Write},
    net,
};

pub fn handle_client(stream: &mut net::TcpStream) {
    let mut buf: [u8; 1024] = [0; 1024];
    let Ok(addr) = stream.peer_addr() else {
        println!("Failed to receive connection");
        stream
            .shutdown(net::Shutdown::Both)
            .expect("Failed to close connection");
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

        buf = [0; 1024];
    }
}

fn main() {
    let listener: net::TcpListener = cce2e_rs::start_listening();

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream_info) => handle_client(&mut stream_info),
            Err(err) => {
                println!("Failed to connect, {}", err);
                return;
            }
        }
    }
}
