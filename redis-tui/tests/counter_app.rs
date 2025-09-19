// crate insta for snapshot testing
// use ratatui::{backend::TestBackend, Terminal};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::TestBackend, Terminal};
use redis_tui::counter_app::{App, EventReader};
use std::{sync::{mpsc, Arc, Mutex}, thread, time::Duration};

#[test]
#[ignore]
fn simple_hello_world() {
    /* let backend = TestBackend::new(30, 10);
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
    ]); */
}

#[test]
fn counter_app_start_up() {
    let backend = TestBackend::new(30, 10);
    let terminal = Terminal::new(backend).unwrap();
    let (tx, rx) = mpsc::channel();
    let event_reader = TestEventReader { receiver: rx };
    let app = Arc::new(Mutex::new(App::new(terminal, event_reader)));
    let app_clone = Arc::clone(&app);
    thread::spawn(move || {
        app_clone.lock().unwrap().run().unwrap();
    });

    thread::sleep(Duration::from_millis(1));
    let event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    tx.send(Event::Key(event)).unwrap();
    println!("{:?}", app.lock().unwrap().backend().buffer())
}

#[derive(Debug)]
struct TestEventReader {
    receiver: mpsc::Receiver<Event>
}

impl EventReader for TestEventReader {
    fn read(&self) -> std::io::Result<crossterm::event::Event> {
        Ok(self.receiver.recv().expect("Error reading test event"))
    }
}
