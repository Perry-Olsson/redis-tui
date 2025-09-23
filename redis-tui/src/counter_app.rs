use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::CompletedFrame;
use ratatui::{
    prelude::Backend,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
    Terminal
};

pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
struct App {
    state: AppState
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.state.exit {
            self.draw_frame(terminal)?;
            self.read_and_handle_events()?;
        }
        Ok(())
    }

    fn draw_frame<'a, T: Backend>(&mut self, terminal: &'a mut Terminal<T>) -> io::Result<CompletedFrame<'a>> {
        terminal.draw(|frame| frame.render_widget(&self.state, frame.area()))
    }

    fn read_and_handle_events(&mut self) -> io::Result<()> {
        handle_event(&mut self.state, event::read()?);
        Ok(())
    }

}

fn handle_event(state: &mut AppState, event: Event) {
    match event {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(state, key_event)
        },
        _ => {}
    };
}

fn handle_key_event(state: &mut AppState, event: KeyEvent) {
    match event.code {
        KeyCode::Left => state.decremenet_counter(),
        KeyCode::Right => state.increment_counter(),
        KeyCode::Char('q') => state.exit(),
        _ => {}
    }
}


#[derive(Debug, Default)]
struct AppState {
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
pub mod test_tui {
    use super::*;
    use ratatui::backend::TestBackend;

    pub struct TestTui {
        app: App,
        terminal: Terminal<TestBackend>
    }

    impl TestTui {
        pub fn new(width: u16, height: u16) -> TestTui {
            let backend = TestBackend::new(width, height);
            let terminal = Terminal::new(backend).unwrap();
            let app = App::default();
            TestTui { app, terminal }
        }
        pub fn draw(&mut self) {
            self.app.draw_frame(&mut self.terminal).unwrap();
        }

        pub fn handle_event(&mut self, event: Event) {
            handle_event(&mut self.app.state, event);
        }

        pub fn backend(&self) -> &TestBackend {
            self.terminal.backend()
        }
    }
}
