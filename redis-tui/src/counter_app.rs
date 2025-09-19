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

impl AppState {
    fn exit(&mut self) {
        self.exit = true;
    }

    fn increment_counter(&mut self) {
        self.counter += 1;
    }
    
    fn decremenet_counter(&mut self) {
        self.counter -= 1;
    }
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
                handle_key_event(&mut self.state, key_event)
            },
            _ => {}
        };
        Ok(())
    }
}

fn handle_key_event(state: &mut AppState, event: KeyEvent) {
    match event.code {
        KeyCode::Left => state.decremenet_counter(),
        KeyCode::Right => state.increment_counter(),
        KeyCode::Char('q') => state.exit(),
        _ => {}
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

    #[test]
    fn handle_key_event_test() -> io::Result<()> {
        println!("hello");
        let mut app = AppState::default();
        handle_key_event(&mut app, KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        handle_key_event(&mut app, KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        handle_key_event(&mut app, KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
