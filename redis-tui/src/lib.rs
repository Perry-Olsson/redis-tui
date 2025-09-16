pub mod counter_app;

use std::io;

pub fn run() -> io::Result<()> {
    counter_app::run()
}
