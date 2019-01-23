extern crate indexmap;
extern crate num_traits;

pub mod floyd_warshall;
pub mod graph;

mod algorithm;
mod request;
mod response;

use crate::algorithm::Algorithm;
use crate::request::Request;
use crate::response::Response;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::io::BufRead;
use std::str::FromStr;

/// A trait group for `IndexMap`'s structure.
pub trait IndexMapTrait: Eq + Hash {}

/// Implement the `IndexMap` for all types satisfying bounds.
impl<N> IndexMapTrait for N where N: Eq + Hash {}

pub struct ExchangeRatePath<I: BufRead> {
    input: I,
}

impl<I: BufRead> ExchangeRatePath<I> {
    /// Create a new instance of ExchangeRatePath structure.
    pub fn new(input: I) -> Self {
        Self { input }
    }

    pub fn run<N, E>(&mut self)
    where
        N: Clone + Display + Ord + FromStr + IndexMapTrait + Debug,
        <N as FromStr>::Err: Debug,
        E: Clone + Display + Copy + Num + PartialOrd + FromStr + Debug,
        <E as FromStr>::Err: Debug,
    {
        let request = self.form_request::<N, E>();
        let response = Self::process_request::<N, E>(request);
        Self::write_response(response);
    }

    fn form_request<N, E>(&mut self) -> Request<N, E>
    where
        N: Clone + Display + Ord + FromStr + IndexMapTrait + Debug,
        <N as FromStr>::Err: Debug,
        E: Clone + Display + Copy + Num + PartialOrd + FromStr,
        <E as FromStr>::Err: Debug,
    {
        Request::<N, E>::read_from(&mut self.input)
    }

    fn process_request<N, E>(request: Request<N, E>) -> Response<N, E>
    where
        N: Clone + Display + Ord + FromStr + IndexMapTrait + Debug,
        <N as FromStr>::Err: Debug,
        E: Clone + Display + Copy + Num + PartialOrd + FromStr + Debug,
        <E as FromStr>::Err: Debug,
    {
        Algorithm::<N, E, u32>::process(&request)
    }

    fn write_response<N, E>(response: Response<N, E>)
    where
        N: Clone + Display + Ord + FromStr + IndexMapTrait + Debug,
        <N as FromStr>::Err: Debug,
        E: Clone + Display + Copy + Num + PartialOrd + FromStr + Debug,
        <E as FromStr>::Err: Debug,
    {
        print!("{}", response.get_output());
    }
}

#[cfg(test)]
mod tests {}
