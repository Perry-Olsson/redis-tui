use redis_testing::run;

fn main() {
    run().unwrap_or_else(|e| println!("{}", e))
}
