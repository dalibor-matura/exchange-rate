//! Price Update.

extern crate chrono;

// use crate::graph::Graph;
use self::Items::*;
use chrono::{DateTime, FixedOffset};
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

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

pub struct PriceUpdate<N, E>
where
    N: Clone + Ord + FromStr + Eq + Hash,
    <N as FromStr>::Err: Debug,
    E: Clone + Copy + Num + PartialOrd + FromStr,
    <E as FromStr>::Err: Debug,
{
    timestamp: DateTime<FixedOffset>,
    exchange: N,
    source_currency: N,
    destination_currency: N,
    forward_factor: E,
    backward_factor: E,
}

impl<N, E> PriceUpdate<N, E>
where
    N: Clone + Ord + FromStr + Eq + Hash,
    <N as FromStr>::Err: Debug,
    E: Clone + Copy + Num + PartialOrd + FromStr,
    <E as FromStr>::Err: Debug,
{
    /// Create a new instance of `PriceUpdate` structure.
    pub fn new(
        timestamp: DateTime<FixedOffset>,
        exchange: N,
        source_currency: N,
        destination_currency: N,
        forward_factor: E,
        backward_factor: E,
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

    /// Get Index identifying current instance by its primary keys.
    pub fn get_index(&self) -> (N, N, N) {
        (
            self.exchange.clone(),
            self.source_currency.clone(),
            self.destination_currency.clone(),
        )
    }

    /// Get timestamp.
    pub fn get_timestamp(&self) -> &DateTime<FixedOffset> {
        &self.timestamp
    }

    pub fn get_exchange(&self) -> &N {
        &self.exchange
    }

    pub fn get_source_currency(&self) -> &N {
        &self.source_currency
    }

    pub fn get_destination_currency(&self) -> &N {
        &self.destination_currency
    }

    pub fn get_forward_factor(&self) -> &E {
        &self.forward_factor
    }

    pub fn get_backward_factor(&self) -> &E {
        &self.backward_factor
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
    pub fn parse_line(line: &String) -> Result<PriceUpdate<N, E>, Vec<String>> {
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

        let forward_factor = values[&ForwardFactor].parse::<E>();
        if forward_factor.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &ForwardFactor
            ));
        }

        let backward_factor = values[&BackwardFactor].parse::<E>();
        if backward_factor.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &BackwardFactor
            ));
        }

        // Making the rest of values uppercase to be more robust.
        let exchange = values[&Exchange].to_uppercase().parse::<N>();
        if exchange.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &Exchange
            ));
        }

        let source_currency = values[&SourceCurrency].to_uppercase().parse::<N>();
        if source_currency.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &SourceCurrency
            ));
        }

        let destination_currency = values[&DestinationCurrency].to_uppercase().parse::<N>();
        if destination_currency.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &DestinationCurrency
            ));
        }

        // Continue only if all values were parsed successfully (no errors are present).
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self::new(
            timestamp.unwrap(),
            exchange.unwrap(),
            source_currency.unwrap(),
            destination_currency.unwrap(),
            forward_factor.unwrap(),
            backward_factor.unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::request::price_update::Items::*;
    use crate::request::price_update::PriceUpdate;

    #[test]
    fn parse_line() {
        let line = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009";
        let price_update = PriceUpdate::<String, f32>::parse_line(&line.to_string());

        // Test that the line was parsed properly.
        assert!(price_update.is_ok());

        // It is safe to unwrap now.
        let price_update = price_update.unwrap();

        // Test properly parsed line items.
        assert_eq!(
            price_update.timestamp.to_rfc3339(),
            String::from("2017-11-01T09:42:23+00:00")
        );
        assert_eq!(price_update.exchange, "KRAKEN");
        assert_eq!(price_update.source_currency, "BTC");
        assert_eq!(price_update.destination_currency, "USD");
        assert_eq!(price_update.forward_factor, 1000.0);
        assert_eq!(price_update.backward_factor, 0.0009);
    }

    #[test]
    fn parse_line_with_missing_values() {
        let line = "";
        let price_update = PriceUpdate::<String, f32>::parse_line(&line.to_string());

        // Test that the line could not be parsed properly.
        assert!(price_update.is_err());

        // Unwrap errors as they should exist.
        let mut errors = price_update.err().unwrap();

        // Test that all errors are present.
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", BackwardFactor)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", ForwardFactor)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", DestinationCurrency)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", SourceCurrency)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", Exchange)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", Timestamp)
        );

        // No other error is expected.
        assert!(errors.pop().is_none());
    }

    #[test]
    fn parse_line_with_parse_errors() {
        let line = String::from(
            "201--11-01T09:42:23+00:00 KRAKEN BTC USD thousand zero-point-something-small",
        );
        let price_update = PriceUpdate::<String, f32>::parse_line(&line);

        // Test that the line could not be parsed properly.
        assert!(price_update.is_err());

        // Unwrap errors as they should exist.
        let mut errors = price_update.err().unwrap();

        // Test that all errors are present.
        assert_eq!(
            errors.pop().unwrap(),
            format!(
                "The line item <{}> can not be parsed (wrong format)!",
                BackwardFactor
            )
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!(
                "The line item <{}> can not be parsed (wrong format)!",
                ForwardFactor
            )
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!(
                "The line item <{}> can not be parsed (wrong format)!",
                Timestamp
            )
        );

        // No other error is expected.
        assert!(errors.pop().is_none());
    }
}
