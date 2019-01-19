//! PriceUpdate.

extern crate chrono;

use self::Items::*;
use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;
use std::fmt;

#[derive(Eq, PartialEq, Hash)]
pub enum Items {
    Timestamp,
    Exchange,
    SourceCurrency,
    DestinationCurrency,
    ForwardFactor,
    BackwardFactor,
}

impl Items {
    pub fn get_label(&self) -> String {
        match self {
            Timestamp => "timestamp".to_string(),
            Exchange => "exchange".to_string(),
            SourceCurrency => "source_currency".to_string(),
            DestinationCurrency => "destination_currency".to_string(),
            ForwardFactor => "forward_factor".to_string(),
            BackwardFactor => "backward_factor".to_string(),
        }
    }
}

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_label())
    }
}

pub struct PriceUpdate {
    timestamp: DateTime<FixedOffset>,
    exchange: String,
    source_currency: String,
    destination_currency: String,
    forward_factor: f32,
    backward_factor: f32,
}

impl PriceUpdate {
    /// Create a new instance of `PriceUpdate` structure.
    fn new(
        timestamp: DateTime<FixedOffset>,
        exchange: String,
        source_currency: String,
        destination_currency: String,
        forward_factor: f32,
        backward_factor: f32,
    ) -> Self {
        Self {
            timestamp,
            exchange,
            source_currency,
            destination_currency,
            forward_factor,
            backward_factor,
        }
    }

    /// Parse input line and form a new `PriceUpdate` struct from it.
    ///
    /// # `line` format
    ///
    /// <timestamp> <exchange> <source_currency> <destination_currency> <forward_factor> <backward_factor>
    ///
    /// ## Example
    ///
    /// 2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
    pub fn parse_line(line: &String) -> Result<PriceUpdate, Vec<String>> {
        let mut iter = line.split_whitespace();
        let mut values = HashMap::new();
        let mut errors: Vec<String> = Vec::new();

        // Collect raw values.
        for item in &[
            Timestamp,
            Exchange,
            SourceCurrency,
            DestinationCurrency,
            ForwardFactor,
            BackwardFactor,
        ] {
            let value: Option<&str> = iter.next();

            match value {
                Some(s) => {
                    values.insert(item, s);
                }
                None => {
                    errors.push(format!("The line item <{}> is missing!", item));
                }
            }
        }

        // Continue only if none of the collected values is missing (no errors are present).
        if !errors.is_empty() {
            return Err(errors);
        }

        // Parse values.
        let timestamp = DateTime::parse_from_rfc3339(values[&Timestamp]);
        if timestamp.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &Timestamp
            ));
        }

        let forward_factor = values[&ForwardFactor].parse::<f32>();
        if forward_factor.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &ForwardFactor
            ));
        }

        let backward_factor = values[&ForwardFactor].parse::<f32>();
        if backward_factor.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &BackwardFactor
            ));
        }

        // Continue only if all values were parsed successfully (no errors are present).
        if !errors.is_empty() {
            return Err(errors);
        }

        // Get the rest of `String` values;
        let exchange = values[&Exchange];
        let source_currency = values[&SourceCurrency];
        let destination_currency = values[&DestinationCurrency];

        Ok(Self::new(
            timestamp.unwrap(),
            exchange.to_string(),
            source_currency.to_string(),
            destination_currency.to_string(),
            forward_factor.unwrap(),
            backward_factor.unwrap(),
        ))
    }
}
