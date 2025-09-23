fn main() {
    redis_tui::run().unwrap_or_else(|e| println!("{}", e))
}
