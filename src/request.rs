//! Exchange Rate Path Request.

use self::exchange_rate_request::{ExchangeRateRequest, ExchangeRateRequestIndex};
use self::price_update::{PriceUpdate, PriceUpdateIndex};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::io::BufRead;

mod exchange_rate_request;
mod price_update;

/// Exchange Rate Path Request structure.
pub struct Request {
    price_updates: HashMap<PriceUpdateIndex, PriceUpdate>,
    rate_requests: HashMap<ExchangeRateRequestIndex, ExchangeRateRequest>,
}

impl Request {
    /// Create a new instance of empty `Request` structure.
    fn new() -> Self {
        let price_updates = HashMap::new();
        let rate_requests = HashMap::new();

        Self {
            price_updates,
            rate_requests,
        }
    }

    pub fn read_from<T: BufRead>(input: &mut T) -> Self {
        let mut request = Self::new();

        // Read all input and process it.
        for line in input.lines() {
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
                ExchangeRateRequest::LINE_TYPE => match ExchangeRateRequest::parse_line(line) {
                    Ok(rate_request) => self.add_rate_request(rate_request),
                    // The errors handling can be done better. Probably using logging mechanism
                    // or just outputting it to the `std::io::stderr`, letting the process continue
                    // and thus being more robust.
                    Err(errors) => panic!(
                        "Errors occurred while processing input lines, errors: {:?}!",
                        errors
                    ),
                },
                _ => match PriceUpdate::parse_line(line) {
                    Ok(price_update) => self.add_price_update(price_update),
                    // The errors handling can be done better. Probably using logging mechanism
                    // or just outputting it to the `std::io::stderr`, letting the process continue
                    // and thus being more robust.
                    Err(errors) => panic!(
                        "Errors occurred while processing input lines, errors: {:?}!",
                        errors
                    ),
                },
            }
        }
    }

    fn add_rate_request(&mut self, rate_request: ExchangeRateRequest) {
        // Use the latest.
        self.rate_requests
            .insert(rate_request.get_index(), rate_request);
    }

    fn add_price_update(&mut self, price_update: PriceUpdate) {
        let entry = self.price_updates.entry(price_update.get_index());

        match entry {
            // The 'PriceUpdate' with the same id already exists in the collection (`HashMap`).
            Occupied(o) => {
                let existing = o.get();

                // The newly provided `PriceUpdate` is more recent and thus
                // it should replace the already existing entry.
                if price_update.get_timestamp() > existing.get_timestamp() {
                    // Replace the existing entry with a new one (the new `PriceUpdate`).
                    *o.into_mut() = price_update;
                }
            }
            // The 'PriceUpdate' with the same id is not yet present in the collection, insert it.
            Vacant(v) => {
                v.insert(price_update);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::request::Request;
    use std::io::BufReader;

    #[test]
    fn process_line() {
        let mut request = Request::new();

        // Test adding ProcessUpdate line.
        let price_update_line =
            String::from("2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009");
        request.process_line(&price_update_line);

        // Test counts of PriceUpdate items and ExchangeRateRequest items.
        assert_eq!(request.price_updates.len(), 1);
        assert_eq!(request.rate_requests.len(), 0);

        // Test adding ExchangeRateRequest line.
        let price_update_line = String::from("EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX ETH");
        request.process_line(&price_update_line);

        // Test counts of PriceUpdate items and ExchangeRateRequest items.
        assert_eq!(request.price_updates.len(), 1);
        assert_eq!(request.rate_requests.len(), 1);
    }

    #[test]
    fn read_from() {
        let text_input = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 KRAKEN ETH USD 100.0 0.001
EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX ETH
EXCHANGE_RATE_REQUEST GDAX BTC KRAKEN USD"
            .as_bytes();

        // Test creation of Request from multiline text.
        let mut input = BufReader::new(text_input);
        let mut request = Request::read_from(&mut input);

        // Test counts of PriceUpdate items and ExchangeRateRequest items.
        assert_eq!(request.price_updates.len(), 2);
        assert_eq!(request.rate_requests.len(), 2);
    }

    #[test]
    fn read_from_skip_empty_lines() {
        let text_input = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
\n
2018-11-01T09:42:23+00:00 KRAKEN ETH USD 100.0 0.001
\n\n
EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX ETH

EXCHANGE_RATE_REQUEST KRAKEN ETH GDAX USD

EXCHANGE_RATE_REQUEST KRAKEN COIN GDAX USD

EXCHANGE_RATE_REQUEST GDAX BTC KRAKEN USD"
            .as_bytes();

        // Test creation of Request from multiline text containing empty or whitespace-only lines.
        let mut input = BufReader::new(text_input);
        let mut request = Request::read_from(&mut input);

        // Test counts of PriceUpdate items and ExchangeRateRequest items.
        assert_eq!(request.price_updates.len(), 2);
        assert_eq!(request.rate_requests.len(), 4);
    }
}
