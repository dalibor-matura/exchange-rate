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
use std::fmt::Debug;
use std::io::{self, BufRead};
use std::str::FromStr;

pub struct ExchangeRatePath<I: BufRead> {
    input: I,
}

impl<I: BufRead> ExchangeRatePath<I> {
    /// Create a new instance of ExchangeRatePath structure.
    pub fn new(input: I) -> Self {
        Self { input }
    }

    pub fn run<T>(&mut self)
    where
        T: Clone + Copy + Num + PartialOrd + FromStr,
        <T as FromStr>::Err: Debug,
    {
        let request = self.form_request::<T>();
        let response = Self::process_request::<T>(request);
        Self::write_response(response);
    }

    fn form_request<T>(&mut self) -> Request<T>
    where
        T: Clone + Copy + Num + PartialOrd + FromStr,
        <T as FromStr>::Err: Debug,
    {
        Request::<T>::read_from(&mut self.input)
    }

    fn process_request<T>(request: Request<T>) -> Response
    where
        T: Clone + Copy + Num + PartialOrd + FromStr,
        <T as FromStr>::Err: Debug,
    {
        Algorithm::<u32, T>::process(&request)
    }

    fn write_response(response: Response) {}
}

#[cfg(test)]
mod tests {}
