use cce2e_rs::arg::GlobalArg;
use clap::Parser;

fn main() -> color_eyre::Result<()> {
    let _ = GlobalArg::parse();

    color_eyre::install()?;
    ratatui::run(cce2e_rs::app)?;

    Ok(())
}
