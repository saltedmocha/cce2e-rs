use std::{
    ffi,
    io::{self, Read, Write},
    net, process, str,
};

pub fn get_name() -> String {
    let mut buf: String = String::new();
    let input: io::Stdin = io::stdin();

    print!("Please enter your name: ");
    let _ = io::stdout().flush();
    if let Err(err) = input.read_line(&mut buf) {
        println!("Failed to get name, {}", err);
        return get_name();
    };

    buf.trim_matches('\n').to_string()
}

pub fn connect_to_host(stream_info: &args::AppArgs) -> net::TcpStream {
    let domain: String = stream_info.get_hostname();
    if let Ok(stream) = net::TcpStream::connect(&domain) {
        println!("Info: Connected to {}", &domain);
        return stream;
    };

    println!("Failed to connect to {}", &domain);
    process::exit(1);
}

fn buf_to_str(buf: &[u8]) -> Result<&str, str::Utf8Error> {
    match ffi::CStr::from_bytes_until_nul(buf) {
        Ok(cstr) => cstr.to_str(),
        Err(_) => str::from_utf8(buf),
    }
}

fn read_input(name: &String) -> Option<(usize, [u8; 2048])> {
    print!("{}: ", name);
    let _ = io::stdout().flush();

    let mut buf: [u8; 2048] = [0; 2048];
    let mut input: io::Stdin = io::stdin();

    let Ok(input_size) = input.read(&mut buf) else {
        println!("Failed to read input");
        return None;
    };

    Some((input_size, buf))
}

pub fn handle_server(name: &String, stream: &mut net::TcpStream) {
    let mut buf: [u8; 2048] = [0; 2048];
    let Ok(addr) = stream.peer_addr() else {
        println!("Failed to receive connection");
        let _ = stream.shutdown(net::Shutdown::Both);
        return;
    };

    loop {
        let Some((size, res)) = read_input(name) else {
            continue;
        };

        if let Err(err) = stream.write_all(&res[..size]) {
            println!("Failed to write data, {}", err);
            return;
        };

        print!("Recv: ");
        let Ok(stream_read) = stream.read(&mut buf) else {
            println!("Failed to read input data");
            return;
        };

        if stream_read == 0 {
            println!("Connection {} closed/disconnected", addr);
            return;
        }

        println!("{}", buf_to_str(&buf).unwrap_or("Failed to parse input"));

        buf = [0; 2048];
    }
}

pub mod args;
