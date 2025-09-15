pub mod counter_app;
pub mod hello_world;

use std::io;

pub fn run() -> io::Result<()> {
    counter_app::run()
}
