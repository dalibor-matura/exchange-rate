//! Floyd-Warshall result.

use crate::graph::Graph;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::cmp::{Eq, Ord};
use std::fmt;
use std::hash::Hash;

/// Floyd-Warshall algorithm Result structure.
///
/// # 'FloydWarshallResult' struct is parametrized over:
///
/// - Node index / label `N`.
/// - Number type `E` giving a weight to edges.
pub struct FloydWarshallResult<N, E>
where
    N: Eq + Copy + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    pub path: Graph<N, E>,
    pub next: Graph<N, N>,
}

impl<N, E> FloydWarshallResult<N, E>
where
    N: Eq + Copy + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    /// Create a new instance of FloydWarshallResult structure.
    pub(super) fn new(path: Graph<N, E>, next: Graph<N, N>) -> Self {
        Self { path, next }
    }

    /// Get path rate.
    ///
    /// The path is specified by starting node `a` and end node `b`.
    ///
    /// This is just a wrapper around `Graph::edge_weight()` method.
    pub fn get_path_rate(&self, a: N, b: N) -> Option<&E> {
        self.path.edge_weight(a, b)
    }

    pub fn collect_path_nodes(&self, a: N, b: N) -> Vec<N> {
        // Get the first path step.
        let next = self.next.edge_weight(a, b);

        // Return empty vector if there is no path between `a` and `b`.
        if next.is_none() {
            return vec![];
        }

        // It is safe to unwrap now.
        let mut next = *next.unwrap();

        // Initiate nodes list with `a`.
        let mut nodes = vec![a];

        while next != b {
            nodes.push(next);

            let new_next = self.next.edge_weight(next, b);

            match new_next {
                Some(&n) => next = n,
                None => break,
            }
        }

        // Close nodes list with `b`.
        nodes.push(b);

        nodes
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

        let _result: FloydWarshallResult<&str, f32> = FloydWarshallResult::new(path, rate);
    }

    #[test]
    fn get_path_rate() {
        let mut path: Graph<&str, f32> = Graph::new();
        let rate: Graph<&str, &str> = Graph::new();

        let a = "a";
        let b = "b";
        let weight = 12.4;

        path.add_edge(a, b, weight);

        let result: FloydWarshallResult<&str, f32> = FloydWarshallResult::new(path, rate);

        assert_eq!(result.get_path_rate(a, b), Some(&weight));
    }

    #[test]
    fn collect_path_nodes() {
        let mut path: Graph<&str, f32> = Graph::new();
        let mut rate: Graph<&str, &str> = Graph::new();

        let w_a_b = 1.2;
        let w_a_c = 4.2;
        let w_b_d = 0.2;
        let w_c_d = 3.3;

        path.add_edge("a", "b", w_a_b);
        path.add_edge("a", "c", w_a_c);
        path.add_edge("b", "d", w_b_d);
        path.add_edge("c", "d", w_c_d);
        path.add_edge("a", "d", w_a_b + w_b_d);

        rate.add_edge("a", "b", "b");
        rate.add_edge("a", "c", "c");
        rate.add_edge("b", "d", "d");
        rate.add_edge("c", "d", "d");
        rate.add_edge("a", "d", "d");

        rate.add_edge("a", "d", "b");
        rate.add_edge("b", "d", "d");

        let result: FloydWarshallResult<&str, f32> = FloydWarshallResult::new(path, rate);

        // Test that path rate and nodes are returned right for the path `(a, b)`.
        assert_eq!(result.get_path_rate("a", "b"), Some(&w_a_b));
        assert_eq!(result.collect_path_nodes("a", "b"), vec!["a", "b"]);

        // Test that path rate and nodes are returned right for the path `(a, d)`.
        assert_eq!(result.get_path_rate("a", "d"), Some(&(w_a_b + w_b_d)));
        assert_eq!(result.collect_path_nodes("a", "d"), vec!["a", "b", "d"]);
    }
}
