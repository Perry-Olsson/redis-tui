use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{List, Padding, Paragraph, Wrap};
use ratatui::CompletedFrame;
use ratatui::{
    prelude::Backend,
    symbols::border,
    widgets::{Block, Widget},
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

    fn draw_frame<'a, T: Backend>(&self, terminal: &'a mut Terminal<T>) -> io::Result<CompletedFrame<'a>> {
        terminal.draw(|frame| frame.render_widget(self, frame.area()))
    }

    fn read_and_handle_events(&mut self) -> io::Result<()> {
        handle_event(&mut self.state, event::read()?);
        Ok(())
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(60),
            ])
            .split(area);
        render_connection_info_block(layout[0], buf);
        render_commands_block(layout[1], buf);
        render_results_block(layout[2], buf);
    }
}

fn render_connection_info_block(area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .title("Connection Info ".bold())
        .border_set(border::THICK)
        .render(area, buf);
}

fn render_commands_block(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .padding(Padding::new(1, 0, 0, 0))
        .title("[1]".bold())
        .title("Commands ".bold())
        .border_set(border::THICK);

    List::new(["KEYS *", "HGETALL post:16"])
        .block(block)
        .render(area, buf);
}

fn render_results_block(area: Rect, buf: &mut Buffer) {
    Block::bordered()
        .title("[2]".bold())
        .title("Results ".bold())
        .border_set(border::THICK)
        .render(area, buf);
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
        KeyCode::Char('q') => state.exit(),
        _ => {}
    }
}


#[derive(Debug, Default)]
struct AppState {
    exit: bool
}

impl AppState {
    fn exit(&mut self) {
        self.exit = true;
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
