use clap::Parser;
use std::net;

#[derive(Debug, Parser)]
#[command(name = "cce2e-client")]
#[command(about = "Realtime relay chat server on TCP", long_about = None)]
pub struct AppArgs {
    /// Foreign host address to connect to
    host: String,
    /// Foreign port used
    port: u16,
}

impl AppArgs {
    pub fn get_hostname(&self) -> String {
        format!("{}:{:?}", self.host, self.port)
    }

    pub fn get_host(&self) -> &String {
        &self.host
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn to_ipv4(&self) -> Option<net::Ipv4Addr> {
        let Ok(ip_address) = self.host.parse::<net::Ipv4Addr>() else {
            println!("Failed to parse IP address");
            return None;
        };

        Some(ip_address)
    }
}
