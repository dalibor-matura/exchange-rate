extern crate indexmap;
extern crate num_traits;

pub mod floyd_warshall;
pub mod graph;
pub mod request;
pub mod response;

use crate::request::Request;
use crate::response::Response;
use std::io::{self, BufRead};

pub struct ExchangeRatePath<I: BufRead> {
    input: I,
}

impl<I: BufRead> ExchangeRatePath<I> {
    /// Create a new instance of ExchangeRatePath structure.
    pub fn new(input: I) -> Self {
        Self { input }
    }

    pub fn run(&mut self) {
        let request = self.form_request();
        let response = Self::process_request(request);
        Self::write_response(response);
    }

    fn form_request(&mut self) -> Request {
        Request::read_from(&mut self.input)
    }

    fn process_request(request: Request) -> Response {
        Response {}
    }

    fn write_response(response: Response) {}
}

#[cfg(test)]
mod tests {}
