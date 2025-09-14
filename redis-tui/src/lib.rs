use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

pub fn run() {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw).expect("Failed to draw frame");
        if matches!(event::read().expect("Failed to read event"), Event::Key(_)) {
            break;
        }
    }

    ratatui::restore()
}

pub fn draw(frame: &mut Frame) {
    let text = Text::raw("Hello world!");
    frame.render_widget(text, frame.area());
}

#[cfg(test)]
mod test {
    use super::draw;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn simple_hello_world() {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(draw).expect("Failed to draw");

        terminal.backend().assert_buffer_lines([
            "Hello world!                  ",
            "                              ",
            "                              ",
            "                              ",
            "                              ",
            "                              ",
            "                              ",
            "                              ",
            "                              ",
            "                              ",
        ]);
    }
}
