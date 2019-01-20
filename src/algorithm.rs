//! Exchange Rate Path (ERP) algorithm.

use crate::graph::Graph;
use crate::request::Request;
use crate::response::Response;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::{Eq, PartialOrd};
use std::collections::hash_map::{Entry, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;
use std::str::FromStr;

/// Exchange Rate Path (ERP) Algorithm structure.
pub struct Algorithm<I>
where
    I: Clone + Copy + Num + PartialOrd + FromStr + AddAssign + Eq + Hash,
    <I as FromStr>::Err: Debug,
{
    string_to_index: HashMap<String, I>,
    index_to_string: HashMap<I, String>,
    counter: I,
}

impl<I> Algorithm<I>
where
    I: Clone + Copy + Num + PartialOrd + FromStr + AddAssign + Eq + Hash,
    <I as FromStr>::Err: Debug,
{
    fn new() -> Self {
        let string_to_index = HashMap::<String, I>::new();
        let index_to_string = HashMap::<I, String>::new();
        let counter = I::zero();

        Self {
            string_to_index,
            index_to_string,
            counter,
        }
    }

    /// Get index of the provided `String`.
    ///
    /// If the `String` is not yet indexed, do so and return the new index.
    fn string_to_index(&mut self, s: String) -> I {
        match self.string_to_index.entry(s.clone()) {
            // Return the index for existing entry.
            Entry::Occupied(o) => *o.get(),
            // Insert with a proper index based on counter.
            Entry::Vacant(v) => {
                // Increase the counter here because new index was requested.
                self.counter += I::one();
                // Use counter as a new index.
                *v.insert(self.counter);
                // Update the reverse `HashMap`.
                self.index_to_string.insert(self.counter, s);
                // Return the index.
                self.counter
            }
        }
    }

    /// Get `String` for the provided index.
    ///
    /// Return `Option<String>` as it is possible that there's no `String` with the index.
    fn index_to_string(&self, i: &I) -> Option<&String> {
        self.index_to_string.get(i)
    }

    pub fn process<T>(request: &Request<T>) -> Response
    where
        T: Clone + Copy + Num + PartialOrd + FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut alg = Algorithm::<I>::new();

        Response {}
    }
}

#[cfg(test)]
mod tests {}
