use crate::algorithm::Algorithm;
use crate::request::Request;
use crate::response::Response;
use floyd_warshall_alg::FloydWarshallTrait;
use std::clone::Clone;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io::BufRead;
use std::str::FromStr;

/// A trait group for `IndexMap`'s structure.
pub trait IndexMapTrait: Eq + Hash {}

/// Implement the `IndexMap` for all types satisfying bounds.
impl<N> IndexMapTrait for N where N: Eq + Hash {}

/// `ExchangeRatePath` structure.
///
/// # `ExchangeRatePath<I>` is parameterized over:
///
/// - Index `I` for indexing of nodes `N`.
pub struct ExchangeRatePath<I: BufRead> {
    input: I,
}

impl<I: BufRead> ExchangeRatePath<I> {
    /// Create a new instance of ExchangeRatePath structure.
    ///
    /// # Examples
    /// ```
    /// use exchange_rate::ExchangeRatePath;
    ///
    /// ExchangeRatePath::new(std::io::stdin().lock());
    /// ```
    pub fn new(input: I) -> Self {
        Self { input }
    }

    /// Run the Exchange Rate Path application.
    pub fn run<N, E>(&mut self)
    where
        N: Clone + Display + FromStr + IndexMapTrait + Debug,
        <N as FromStr>::Err: Debug,
        E: Display + FloydWarshallTrait + FromStr + Debug,
        <E as FromStr>::Err: Debug,
    {
        let request = self.form_request::<N, E>();
        let response = Self::process_request::<N, E>(request);
        Self::write_response(response);
    }

    fn form_request<N, E>(&mut self) -> Request<N, E>
    where
        N: Clone + FromStr + IndexMapTrait,
        <N as FromStr>::Err: Debug,
        E: FromStr,
        <E as FromStr>::Err: Debug,
    {
        Request::<N, E>::read_from(&mut self.input)
    }

    fn process_request<N, E>(request: Request<N, E>) -> Response<N, E>
    where
        N: Clone + Display + FromStr + IndexMapTrait + Debug,
        <N as FromStr>::Err: Debug,
        E: Display + FloydWarshallTrait + FromStr + Debug,
        <E as FromStr>::Err: Debug,
    {
        Algorithm::<N, E, u32>::process(&request)
    }

    fn write_response<N, E>(response: Response<N, E>)
    where
        N: Display + Debug,
        E: Display,
    {
        print!("{}", response.get_output());
    }
}

#[cfg(test)]
mod tests {
    use crate::exchange_rate::ExchangeRatePath;
    use crate::request::Request;
    use std::io::BufReader;

    #[test]
    fn new() {
        ExchangeRatePath::new(std::io::stdin().lock());
    }

    #[test]
    fn form_request() {
        // Prepare input.
        let text_input = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 KRAKEN ETH USD 100.0 0.001
EXCHANGE_RATE_REQUEST KRAKEN BTC GDAX ETH
EXCHANGE_RATE_REQUEST GDAX BTC KRAKEN USD"
            .as_bytes();
        let input = BufReader::new(text_input);

        let mut exchange_rate = ExchangeRatePath::new(input);
        let request = exchange_rate.form_request::<String, f32>();
        let price_updates = request.get_price_updates();
        let rate_requests = request.get_rate_requests();

        // Test proper counts.
        assert_eq!(price_updates.len(), 2);
        assert_eq!(rate_requests.len(), 2);
    }

    #[test]
    fn process_request() {
        // Prepare input.
        let text_input = "2017-11-01T09:42:23+00:00 KRAKEN BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 KRAKEN ETH USD 100.0 0.001
EXCHANGE_RATE_REQUEST KRAKEN BTC KRAKEN USD
EXCHANGE_RATE_REQUEST KRAKEN ETH KRAKEN BTC"
            .as_bytes();
        let mut input = BufReader::new(text_input);

        let request = Request::<String, f32>::read_from(&mut input);
        let response = ExchangeRatePath::<&[u8]>::process_request::<String, f32>(request);

        let paths = response.get_best_rate_paths();

        // Test proper count.
        assert_eq!(paths.len(), 2);
    }
}
