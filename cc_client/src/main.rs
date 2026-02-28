use std::net;

use cc_client::args;
use clap::Parser;

fn main() {
    let args_val: args::AppArgs = args::AppArgs::parse();
    let mut stream: net::TcpStream = cc_client::connect_to_host(&args_val);

    let name: String = cc_client::get_name();
    cc_client::handle_server(&name, &mut stream);
}
