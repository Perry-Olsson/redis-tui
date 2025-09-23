mod tests;
pub mod counter_app;

pub fn run() -> std::io::Result<()> {
    counter_app::run()
}
