use std::io;

use cce2e_rs::arg::GlobalArg;
use clap::Parser;

fn main() -> io::Result<()> {
    let _ = GlobalArg::parse();

    let _ = ratatui::run(|terminal| cce2e_rs::tui::AppState::default().run(terminal));

    Ok(())
}
