use crossterm::event::{self, Event};
use ratatui::{text::Text, Frame};

pub fn run() {
    run_hello_world();
}

fn run_hello_world() {
    let mut terminal = ratatui::init();

    loop {
        terminal.draw(draw_hello).expect("Failed to draw frame");
        if matches!(event::read().expect("Failed to read event"), Event::Key(_)) {
            break;
        }
    }

    ratatui::restore()
}

pub fn draw_hello(frame: &mut Frame) {
    let text = Text::raw("Hello world!");
    frame.render_widget(text, frame.area());
}

#[cfg(test)]
mod test {
    use super::draw_hello;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn simple_hello_world() {
        let backend = TestBackend::new(30, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(draw_hello).expect("Failed to draw");

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
