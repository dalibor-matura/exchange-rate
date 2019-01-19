//! ExchangeRateRequest.

pub struct ExchangeRateRequest {}

impl ExchangeRateRequest {
    /// Parse input line and form a new `ExchangeRateRequest` struct from it.
    ///
    /// # `line` format
    ///
    /// EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>
    ///
    /// ## Example
    ///
    /// EXCHANGE_RATE_REQUEST KRAKEN BTC ETH
    pub fn parse_line(line: &String) {
        let mut a_iter = line.split_whitespace();

        let a0 = a_iter.next().unwrap().parse::<i32>().expect("parse error");
        let a1 = a_iter.next().unwrap().parse::<i32>().expect("parse error");
        let a2 = a_iter.next().unwrap().parse::<i32>().expect("parse error");
    }
}
