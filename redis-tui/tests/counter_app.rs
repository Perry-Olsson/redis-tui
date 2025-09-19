// crate insta for snapshot testing
// use ratatui::{backend::TestBackend, Terminal};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::TestBackend, Terminal};
use redis_tui::counter_app::{App, EventReader};
use rstest::{fixture, rstest};
use std::{sync::{mpsc::{self, Sender}, Arc, Mutex}, thread, time::Duration};

#[rstest]
fn counter_app_start_up(tui: Tui) {
    let event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    tui.tx.send(Event::Key(event)).unwrap();
    insta::assert_debug_snapshot!(tui.app.lock().unwrap().backend().buffer());
}

#[fixture]
fn tui() -> Tui {
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
    Tui { app, tx }
}

pub struct Tui {
    app: Arc<Mutex<App<TestBackend, TestEventReader>>>,
    tx: Sender<Event>
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
