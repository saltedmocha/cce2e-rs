use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "cce2e")]
#[command(about = "An telnet copy", long_about = None)]
pub struct AppArgs {
    /// Force to use IPv4
    #[arg(short = '4', long)]
    v4: bool,
    /// Force to use IPv4
    #[arg(short = '6', long)]
    v6: bool,
    /// Host / IP address to connect to
    host: String,
    /// Port used on connecting server
    port: u16,
}

impl AppArgs {}
