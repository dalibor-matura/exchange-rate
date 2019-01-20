use exchange_rate_path::ExchangeRatePath;
use std::io;

fn main() {
    ExchangeRatePath::new(io::stdin().lock()).run();
}
