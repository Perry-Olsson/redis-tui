use std::io::{self, Stdout};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::{Backend, CrosstermBackend}, style::Stylize, symbols::border, text::{Line, Text}, widgets::{Block, Paragraph, Widget}, Terminal
};

pub fn run() -> io::Result<()> {
    let app_result = App::default().run();
    ratatui::restore();
    app_result
}

pub trait EventReader {
    fn read(&self) -> std::io::Result<Event>;
}

pub struct CrosstermEventReader {}

impl EventReader for CrosstermEventReader {
    fn read(&self) -> std::io::Result<Event> {
        event::read()
    }
}

#[derive(Debug)]
pub struct App<B: Backend, E: EventReader> {
    terminal: Terminal<B>,
    event_reader: E,
    state: AppState
}


#[derive(Debug, Default)]
pub struct AppState {
    counter: u8,
    exit: bool
}

impl App<CrosstermBackend<Stdout>, CrosstermEventReader> {
    pub fn default() -> App<CrosstermBackend<Stdout>, CrosstermEventReader> {
        App::new(ratatui::init(), CrosstermEventReader{})
    }
}

impl<B: Backend, T: EventReader> App<B, T> {
    pub fn new(terminal: Terminal<B>, event_reader: T) -> App<B, T> {
        App { terminal, event_reader, state: AppState::default() }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while !self.state.exit {
            self.terminal.draw(|frame| frame.render_widget(&self.state, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn backend(&self) -> &B {
        self.terminal.backend()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match self.event_reader.read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Left => self.decremenet_counter(),
            KeyCode::Right => self.increment_counter(),
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.state.exit = true;
    }

    fn increment_counter(&mut self) {
        self.state.counter += 1;
    }
    
    fn decremenet_counter(&mut self) {
        self.state.counter -= 1;
    }
}

impl Widget for &AppState {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let title = Line::from(" Counter App Tutorial ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".green().bold(),
            " Increment ".into(),
            "<Right>".green().bold(),
            " Quit ".into(),
            "<Q> ".green().bold()
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
                "Value: ".into(),
                self.counter.to_string().yellow()
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{buffer::Buffer, layout::Rect, style::Style};

    #[test]
    #[ignore]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.state.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    #[ignore]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.state.counter, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.state.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.state.exit);

        Ok(())
    }
}
