use crossterm::event::KeyCode;
use redis_tui::{run, draw};
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
