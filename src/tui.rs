use crossterm::event;
use ratatui::{DefaultTerminal, Frame, layout, style::Stylize, symbols, text, widgets};
use std::io;

#[derive(Debug, Default)]
pub struct AppState {
    pub counter: u8,
    pub exit: bool,
}

impl AppState {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_event(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == event::KeyEventKind::Press => {
                todo!()
            }
            _ => {}
        };

        Ok(())
    }
}

impl widgets::Widget for &AppState {
    fn render(self, area: layout::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title: text::Line<'_> = text::Line::from(" Counter App Tutorial ".bold());
        let instructions: text::Line<'_> = text::Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block: widgets::Block<'_> = widgets::Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(symbols::border::THICK);

        let counter_text: text::Line<'_> =
            text::Line::from(vec!["Value: ".into(), self.counter.to_string().yellow()]);

        widgets::Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer, style::Style, widgets::Widget};

    #[test]
    fn render() {
        let app = AppState::default();
        let mut buf = buffer::Buffer::empty(layout::Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = buffer::Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(layout::Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(layout::Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(layout::Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(layout::Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(layout::Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
