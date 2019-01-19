//! Exchange Rate Path Request.

use self::exchange_rate_request::ExchangeRateRequest;
use self::price_update::PriceUpdate;
use std::io;

mod exchange_rate_request;
mod price_update;

/// Exchange Rate Path Request structure.
pub struct Request {
    price_updates: Vec<PriceUpdate>,
    rate_requests: Vec<ExchangeRateRequest>,
}

impl Request {
    /// Create a new instance of empty `Request` structure.
    fn new() -> Self {
        let price_updates = Vec::new();
        let rate_requests = Vec::new();

        Self {
            price_updates,
            rate_requests,
        }
    }

    pub fn read_from_stdin() -> Self {
        let request = Request::new();

        let mut s = String::new();

        io::stdin().read_line(&mut s).expect("IO:Stdin Read error!");

        request
    }
}

// Price update line:
// <timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>

// Exchange rate request line:
// EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>

#[cfg(test)]
mod tests {}
