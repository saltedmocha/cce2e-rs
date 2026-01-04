use ratatui::{DefaultTerminal, Frame};

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Hello world!", frame.area());
}

pub mod arg;
pub mod chat;
pub mod network;
pub mod tui;
pub mod user;
