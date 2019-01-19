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
    pub fn parse_line(line: &String) -> Result<ExchangeRateRequest, Vec<String>> {
        let mut a_iter = line.split_whitespace();

        // Todo: Fill in the real implementation.

        Ok(ExchangeRateRequest {})
    }
}
