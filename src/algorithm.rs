//! Exchange Rate Path (ERP) algorithm.

use crate::graph::Graph;
use crate::request::Request;
use crate::response::Response;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use std::str::FromStr;

/// Exchange Rate Path (ERP) Algorithm structure.
pub struct Algorithm {}

impl Algorithm {
    pub fn process<T>(request: &Request<T>) -> Response
    where
        T: Clone + Copy + Num + PartialOrd + FromStr,
        <T as FromStr>::Err: Debug,
    {
        Response {}
    }
}

#[cfg(test)]
mod tests {}
