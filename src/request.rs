//! Exchange Rate Path Request.

use self::exchange_rate_request::ExchangeRateRequest;
use self::price_update::PriceUpdate;
use std::io::{self, BufRead};

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
        let mut request = Request::new();

        // Read all input and process it.
        for line in io::stdin().lock().lines() {
            if let Ok(s) = line {
                request.process_line(&s);
            }
        }

        request
    }

    fn process_line(&mut self, line: &String) {
        let mut iter = line.split_whitespace();

        // Process the first line item if it exists.
        if let Some(first_item) = iter.next() {
            // Match the line type based on the first line item.
            // The line item is used as uppercase to be more robust.
            match first_item.to_uppercase().as_ref() {
                "EXCHANGE_RATE_REQUEST" => match ExchangeRateRequest::parse_line(line) {
                    Ok(rate_request) => self.add_rate_request(rate_request),
                    Err(errors) => (),
                },
                _ => match PriceUpdate::parse_line(line) {
                    Ok(price_update) => self.add_price_update(price_update),
                    Err(errors) => (),
                },
            }
        }
    }

    fn add_rate_request(&mut self, rate_request: ExchangeRateRequest) {
        self.rate_requests.push(rate_request);
    }

    fn add_price_update(&mut self, price_update: PriceUpdate) {
        self.price_updates.push(price_update);
    }
}

#[cfg(test)]
mod tests {}
