use std::{
    io::{Read, Write},
    net,
};

pub struct StreamInfo {
    address: net::SocketAddr,
    connection: net::TcpStream,
}

impl StreamInfo {
    pub fn get_address(self) -> net::SocketAddr {
        self.address
    }

    pub fn get_connection(self) -> net::TcpStream {
        self.connection
    }

    pub fn try_new(stream: net::TcpStream) -> Option<Self> {
        let Ok(host_address) = stream.peer_addr() else {
            println!("Failed to retrieve foreign address");
            return None;
        };

        Some(StreamInfo {
            address: host_address,
            connection: stream,
        })
    }

    pub fn handle_client(&mut self) {
        let mut buf: [u8; 2048] = [0; 2048];
        let Ok(addr) = self.connection.peer_addr() else {
            println!("Failed to receive connection");
            self.connection
                .shutdown(net::Shutdown::Both)
                .expect("Failed to close connection");
            return;
        };

        println!("Received connection from {}", addr);

        loop {
            let Ok(stream_read) = self.connection.read(&mut buf) else {
                println!("Failed to read input data");
                return;
            };

            if stream_read == 0 {
                println!("Connection {} closed/disconnected", addr);
                return;
            }

            if let Err(err) = self.connection.write_all(&buf[..stream_read]) {
                println!("Failed to write data, {}", err);
                return;
            };

            buf = [0; 2048];
        }
    }
}
