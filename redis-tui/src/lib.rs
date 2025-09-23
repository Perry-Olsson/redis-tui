pub mod counter_app;
mod tests;

use std::io;

pub fn run() -> io::Result<()> {
    counter_app::run()
}
