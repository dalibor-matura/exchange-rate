//! Exchange Rate Path Request.

use self::exchange_rate_request::{ExchangeRateRequest, ExchangeRateRequestIndex};
use self::price_update::{PriceUpdate, PriceUpdateIndex};
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::io::{self, BufRead};

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

    pub fn read_from_stdin() -> Self {
        let mut request = Self::new();

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
                ExchangeRateRequest::LINE_TYPE => match ExchangeRateRequest::parse_line(line) {
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
mod tests {}
