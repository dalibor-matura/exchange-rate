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
mod tests {}
