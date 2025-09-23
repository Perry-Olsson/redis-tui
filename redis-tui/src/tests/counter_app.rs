use crate::counter_app::test_tui::{TestTui};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use rstest::{fixture, rstest};

#[rstest]
fn start_up(mut tui: TestTui) {
    tui.draw();
    insta::assert_debug_snapshot!(tui.backend())
}

#[rstest]
fn increment_by_one(mut tui: TestTui) {
    tui.draw();

    let event = KeyEvent::new(KeyCode::Right, KeyModifiers::NONE);
    tui.handle_event(Event::Key(event));

    tui.draw();

    insta::assert_debug_snapshot!(tui.backend())
}

#[fixture]
fn tui() -> TestTui {
    TestTui::new(70, 30)
}
