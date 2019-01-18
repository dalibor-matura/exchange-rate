//! Floyd-Warshall algorithm.

use crate::graph::Graph;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::cmp::{Eq, Ord};
use std::fmt;
use std::hash::Hash;

/// Floyd-Warshall algorithm Result structure.
pub struct FloydWarshallResult<T, N>
where
    T: Eq + Copy + Hash + Ord + fmt::Debug,
    N: Clone + Copy + Num + PartialOrd,
{
    path: Graph<T, N>,
    next: Graph<T, T>,
}

impl<T, N> FloydWarshallResult<T, N>
where
    T: Eq + Copy + Hash + Ord + fmt::Debug,
    N: Clone + Copy + Num + PartialOrd,
{
    /// Create a new instance of FloydWarshallResult structure.
    pub(super) fn new(path: Graph<T, N>, next: Graph<T, T>) -> Self {
        Self { path, next }
    }
}

#[cfg(test)]
mod tests {
    use crate::floyd_warshall::result::FloydWarshallResult;
    use crate::graph::Graph;

    #[test]
    fn new() {
        let path: Graph<&str, f32> = Graph::new();
        let rate: Graph<&str, &str> = Graph::new();

        let result: FloydWarshallResult<&str, f32> = FloydWarshallResult::new(path, rate);
    }
}
