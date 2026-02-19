use std::net;

fn main() {
    let listener: net::TcpListener = cc_server::start_listening();

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream_info) => cc_server::handle_client(&mut stream_info),
            Err(err) => {
                println!("Failed to connect, {}", err);
                return;
            }
        }
    }
}
