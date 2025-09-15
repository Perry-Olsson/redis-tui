use redis_tui::run;

fn main() {
    run().unwrap_or_else(|e| println!("{}", e))
}
