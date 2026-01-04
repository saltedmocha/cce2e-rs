use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Copy, Debug, Default)]
enum UIMode {
    Noui,
    Tui,
    #[default]
    Gui,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
pub struct GlobalArg {
    #[arg(long, value_enum, default_value_t = UIMode::Gui, default_missing_value = "always")]
    ui: UIMode,

    #[arg(short, long, default_value_t = 5050)]
    port: u16,
}
