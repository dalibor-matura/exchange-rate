//! Best Rate Path.

use std::fmt::{Debug, Display};

pub struct BestRatePath<N, E> {
    rate: E,
    path: Vec<(N, N)>,
}

/// Exchange `BestRatePath` structure.
///
/// # `BestRatePath<N, E>` is parameterized over:
///
/// - Identifier data `N`.
/// - Edge weight `E`.
impl<N, E> BestRatePath<N, E>
where
    N: Display + Debug,
    E: Display,
{
    pub fn new(rate: E, path: Vec<(N, N)>) -> Self {
        Self { rate, path }
    }

    pub fn get_rate(&self) -> &E {
        &self.rate
    }

    pub fn get_path(&self) -> &Vec<(N, N)> {
        &self.path
    }

    #[allow(dead_code)]
    pub fn get_start_node(&self) -> Option<&(N, N)> {
        self.path.first()
    }

    #[allow(dead_code)]
    pub fn get_end_node(&self) -> Option<&(N, N)> {
        self.path.last()
    }

    /// Get printable output representing the Best Rated Path.
    ///
    /// # Format
    ///
    /// BEST_RATES_BEGIN <source_exchange> <source_currency> <destination_exchange>
    /// <destination_currency> <rate>
    /// <source_exchange, source_currency>
    /// <exchange, currency>
    /// <exchange, currency>
    /// ...
    /// <destination_exchange, destination_currency>
    /// BEST_RATES_END
    ///
    /// ## Example
    ///
    /// BEST_RATES_BEGIN <a> <b> <g> <h> <10.2>
    /// <a, b>
    /// <c, d>
    /// <e, f>
    /// <g, h>
    /// BEST_RATES_END
    pub fn get_output(&self) -> String {
        let path = self.get_path();

        // Return empty string nn case the path is not valid.
        if path.len() < 2 {
            return String::new();
        }

        let (source_exchange, source_currency) = path.first().unwrap();
        let (destination_exchange, destination_currency) = path.last().unwrap();

        format!(
            "BEST_RATES_BEGIN <{}> <{}> <{}> <{}> <{}>\n\
             {}\
             BEST_RATES_END\n",
            source_exchange,
            source_currency,
            destination_exchange,
            destination_currency,
            self.get_rate(),
            self.get_path_output(),
        )
    }

    /// Get printable output of the path.
    ///
    /// # Format
    ///
    /// <source_exchange, source_currency>
    /// <exchange, currency>
    /// <exchange, currency>
    /// ...
    /// <destination_exchange, destination_currency>
    pub fn get_path_output(&self) -> String {
        let mut output = String::new();

        for (a, b) in self.get_path() {
            output.push_str(&format!("<{}, {}>\n", a, b));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use crate::response::best_rate_path::BestRatePath;

    #[test]
    fn get_path_output() {
        let rate = 10.0;
        let mut path: Vec<(String, String)> = Vec::with_capacity(4);
        path.push(("a".to_string(), "b".to_string()));
        path.push(("c".to_string(), "d".to_string()));
        path.push(("e".to_string(), "f".to_string()));
        path.push(("g".to_string(), "h".to_string()));

        let best_rate_path = BestRatePath::<String, f32>::new(rate, path);

        assert_eq!(
            best_rate_path.get_path_output(),
            String::from("<a, b>\n<c, d>\n<e, f>\n<g, h>\n")
        );
    }

    #[test]
    fn get_output() {
        let rate = 10.2;
        let mut path: Vec<(String, String)> = Vec::with_capacity(4);
        path.push(("a".to_string(), "b".to_string()));
        path.push(("c".to_string(), "d".to_string()));
        path.push(("e".to_string(), "f".to_string()));
        path.push(("g".to_string(), "h".to_string()));

        let best_rate_path = BestRatePath::<String, f32>::new(rate, path);

        assert_eq!(
            best_rate_path.get_output(),
            String::from(
                "BEST_RATES_BEGIN <a> <b> <g> <h> <10.2>\n\
                 <a, b>\n\
                 <c, d>\n\
                 <e, f>\n\
                 <g, h>\n\
                 BEST_RATES_END\n"
            )
        );
    }
}
