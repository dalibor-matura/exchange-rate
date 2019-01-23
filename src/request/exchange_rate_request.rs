//! Exchange Rate Request.

use self::Items::*;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash)]
pub enum Items {
    LineType,
    SourceExchange,
    SourceCurrency,
    DestinationExchange,
    DestinationCurrency,
}

impl Items {
    pub fn get_label(&self) -> String {
        match self {
            LineType => "EXCHANGE_RATE_REQUEST".to_string(),
            SourceExchange => "source_exchange".to_string(),
            SourceCurrency => "source_currency".to_string(),
            DestinationExchange => "source_exchange".to_string(),
            DestinationCurrency => "destination_exchange".to_string(),
        }
    }
}

impl fmt::Display for Items {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_label())
    }
}

/// `ExchangeRateRequest` structure.
///
/// # `ExchangeRateRequest<N>` is parameterized over:
///
/// - Identifier data `N`.
pub struct ExchangeRateRequest<N> {
    source_exchange: N,
    source_currency: N,
    destination_exchange: N,
    destination_currency: N,
}

impl<N> ExchangeRateRequest<N>
where
    N: Clone + FromStr,
    <N as FromStr>::Err: fmt::Debug,
{
    // The type of a line that can be parsed into the `ExchangeRateRequest` structure.
    pub const LINE_TYPE: &'static str = "EXCHANGE_RATE_REQUEST";

    /// Create a new instance of `ExchangeRateRequest` structure.
    pub fn new(
        source_exchange: N,
        source_currency: N,
        destination_exchange: N,
        destination_currency: N,
    ) -> Self {
        Self {
            source_exchange,
            source_currency,
            destination_exchange,
            destination_currency,
        }
    }

    /// Get Index identifying current instance by its primary keys.
    pub fn get_index(&self) -> (N, N, N, N) {
        (
            self.source_exchange.clone(),
            self.source_currency.clone(),
            self.destination_exchange.clone(),
            self.destination_currency.clone(),
        )
    }

    pub fn get_source_exchange(&self) -> &N {
        &self.source_exchange
    }

    pub fn get_source_currency(&self) -> &N {
        &self.source_currency
    }

    pub fn get_destination_exchange(&self) -> &N {
        &self.destination_exchange
    }

    pub fn get_destination_currency(&self) -> &N {
        &self.destination_currency
    }

    /// Parse input line and form a new `ExchangeRateRequest` struct from it.
    ///
    /// # `line` format
    ///
    /// EXCHANGE_RATE_REQUEST <source_exchange> <source_currency> <destination_exchange> <destination_currency>
    ///
    /// ## Example
    ///
    /// EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX ETH
    pub fn parse_line(line: &String) -> Result<ExchangeRateRequest<N>, Vec<String>> {
        let mut iter = line.split_whitespace();
        let mut values = HashMap::new();
        let mut errors: Vec<String> = Vec::new();

        // Collect raw values.
        for item in &[
            LineType,
            SourceExchange,
            SourceCurrency,
            DestinationExchange,
            DestinationCurrency,
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

        // Validate line type.
        let line_type = values[&LineType].to_uppercase();
        if line_type != Self::LINE_TYPE {
            errors.push(format!(
                "The line item type identifier at the beginning of the line {} is wrong!",
                Self::LINE_TYPE
            ));
            return Err(errors);
        }

        // Parse values, also making it all uppercase to be more robust.
        let source_exchange = values[&SourceExchange].to_uppercase().parse::<N>();
        if source_exchange.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &SourceExchange
            ));
        }

        let source_currency = values[&SourceCurrency].to_uppercase().parse::<N>();
        if source_currency.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &SourceCurrency
            ));
        }

        let destination_exchange = values[&DestinationExchange].to_uppercase().parse::<N>();
        if destination_exchange.is_err() {
            errors.push(format!(
                "The line item <{}> can not be parsed (wrong format)!",
                &DestinationExchange
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
            source_exchange.unwrap(),
            source_currency.unwrap(),
            destination_exchange.unwrap(),
            destination_currency.unwrap(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::request::exchange_rate_request::ExchangeRateRequest;
    use crate::request::exchange_rate_request::Items::*;

    #[test]
    fn parse_line() {
        let line = "EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX ETH";
        let rate_request = ExchangeRateRequest::<String>::parse_line(&line.to_string());

        // Test that the line was parsed properly.
        assert!(rate_request.is_ok());

        // It is safe to unwrap now.
        let rate_request = rate_request.unwrap();

        // Test properly parsed line items.
        assert_eq!(rate_request.source_exchange, "KRAKEN");
        assert_eq!(rate_request.source_currency, "BTC");
        assert_eq!(rate_request.destination_exchange, "GDAX");
        assert_eq!(rate_request.destination_currency, "ETH");
    }

    #[test]
    fn parse_line_with_wrong_line_type() {
        let line = "WRONG_LINE_TYPE KRAKEN BTC GDAX ETH";
        let price_update = ExchangeRateRequest::<String>::parse_line(&line.to_string());

        // Test that the line could not be parsed properly.
        assert!(price_update.is_err());

        // Unwrap errors as they should exist.
        let mut errors = price_update.err().unwrap();

        // Test that all errors are present.
        assert_eq!(
            errors.pop().unwrap(),
            format!(
                "The line item type identifier at the beginning of the line {} is wrong!",
                ExchangeRateRequest::<String>::LINE_TYPE
            )
        );

        // No other error is expected.
        assert!(errors.pop().is_none());
    }

    #[test]
    fn parse_line_with_missing_values() {
        let line = "";
        let price_update = ExchangeRateRequest::<String>::parse_line(&line.to_string());

        // Test that the line could not be parsed properly.
        assert!(price_update.is_err());

        // Unwrap errors as they should exist.
        let mut errors = price_update.err().unwrap();

        // Test that all errors are present.
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", DestinationCurrency)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", DestinationExchange)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", SourceCurrency)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", SourceExchange)
        );
        assert_eq!(
            errors.pop().unwrap(),
            format!("The line item <{}> is missing!", LineType)
        );

        // No other error is expected.
        assert!(errors.pop().is_none());
    }
}
