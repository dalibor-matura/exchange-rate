use exchange_rate::ExchangeRatePath;
use std::io;

fn main() {
    ExchangeRatePath::new(io::stdin().lock()).run::<String, f32>();
}
