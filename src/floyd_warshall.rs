//! Floyd-Warshall algorithm.

use self::result::FloydWarshallResult;
use crate::graph::Graph;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::Ordering::{Greater, Less};
use std::cmp::PartialOrd;
use std::cmp::{Eq, Ord};
use std::fmt;
use std::hash::Hash;

pub mod result;

/// Floyd-Warshall algorithm structure.
pub struct FloydWarshall<N: Clone + Copy + Num + PartialOrd> {
    /// Operator to be used for weighted edges.
    op: Box<Fn(N, N) -> N>,
    /// Comparison to be used for weighted paths.
    cmp: Box<Fn(N, N) -> bool>,
    /// Discard loops (e.g. edges starting and ending in the same node) from calculation.
    discard_loops: bool,
}

impl<N: Clone + Copy + Num + PartialOrd> FloydWarshall<N> {
    /// Create a new instance of FloydWarshall structure with default settings.
    ///
    /// #Examples
    ///
    /// ```
    /// use exchange_rate_path::floyd_warshall::FloydWarshall;
    ///
    /// let alg: FloydWarshall<f32> = FloydWarshall::new();
    /// ```
    pub fn new() -> Self {
        // Initialize defaults.
        let add = Box::new(|x: N, y: N| x + y);
        let sharp_less = Box::new(|x: N, y: N| x.partial_cmp(&y).unwrap_or(Greater) == Less);

        Self {
            op: add,
            cmp: sharp_less,
            discard_loops: true,
        }
    }

    /// Create a new instance of FloydWarshall structure with customized settings.
    ///
    /// You can set:
    /// - the `op` (operator) to be used for weighted edges
    /// - the `cmp` (comparison) to be used for weighted paths
    ///
    /// #Examples
    ///
    /// ```
    /// use exchange_rate_path::floyd_warshall::FloydWarshall;
    /// use std::cmp::Ordering::{Greater, Less};
    ///
    /// let mul = Box::new(|x: f32, y: f32| x + y);
    /// let sharp_greater = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Less) == Greater);
    ///
    /// let alg: FloydWarshall<f32> = FloydWarshall::new_customized(mul, sharp_greater);
    /// ```
    pub fn new_customized(op: Box<Fn(N, N) -> N>, cmp: Box<Fn(N, N) -> bool>) -> Self {
        Self::new_fully_customized(op, cmp, true)
    }

    /// Create a new instance of FloydWarshall structure with customized settings.
    ///
    /// You can set:
    /// - the `op` (operator) to be used for weighted edges
    /// - the `cmp` (comparison) to be used for weighted paths
    /// - the `discard_loops` to discard loops (e.g. edges starting and ending in the same node)
    ///   from calculation.
    ///
    /// #Examples
    ///
    /// ```
    /// use exchange_rate_path::floyd_warshall::FloydWarshall;
    /// use std::cmp::Ordering::{Greater, Less};
    ///
    /// let mul = Box::new(|x: f32, y: f32| x + y);
    /// let sharp_greater = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Less) == Greater);
    /// let discard_loops = false;
    ///
    /// let alg: FloydWarshall<f32> = FloydWarshall::new_customized(mul, sharp_greater);
    pub fn new_fully_customized(
        op: Box<Fn(N, N) -> N>,
        cmp: Box<Fn(N, N) -> bool>,
        discard_loops: bool,
    ) -> Self {
        Self {
            op,
            cmp,
            discard_loops,
        }
    }

    pub fn find_paths<T>(&self, graph: &Graph<T, N>) -> FloydWarshallResult<T, N>
    where
        T: Eq + Copy + Hash + Ord + fmt::Debug,
    {
        let mut path: Graph<T, N> = graph.clone();
        let mut next: Graph<T, T> = Graph::with_capacity(graph.node_count(), graph.edge_count());

        // `k` is the "intermediate" node which is currently considered.
        for k in graph.nodes() {
            // `i` is a starting node of the path we try to optimize.
            for i in graph.nodes() {
                // `j` is an end node of the path we try to optimize.
                for j in graph.nodes() {
                    // Skip calculation for loops if requested.
                    if self.discard_loops && Self::unique(vec![k, i, j]) {
                        continue;
                    }

                    // Calculation of a new weight of the path from `i` to `j` through `k`.
                    let left_operand = path.edge_weight(i, k);
                    let right_operand = path.edge_weight(k, j);

                    // There's nothing to calculate if the left `(i, k)` or right `(k, j)` path misses.
                    if left_operand.is_none() || right_operand.is_none() {
                        continue;
                    }

                    // It is safe to unwrap the operands now.
                    let left_operand = left_operand.unwrap();
                    let right_operand = right_operand.unwrap();

                    // Use the weight operator. It can be customized.
                    let new_weight = (self.op)(*left_operand, *right_operand);

                    // The `(i, j)` path already exists.
                    if let Some(&old_weight) = path.edge_weight(i, j) {
                        // Use the comparison. It can be customized.
                        if (self.cmp)(old_weight, new_weight) {
                            path.add_edge(i, j, new_weight);

                            // The algorithm invariants guarantee the edge exists.
                            let direction = next.edge_weight(i, k).unwrap();
                            next.add_edge(i, j, *direction);
                        }
                    } else {
                        // The path was missing so add a new one.
                        path.add_edge(i, j, new_weight);

                        // The algorithm invariants guarantee the edge exists.
                        let direction = next.edge_weight(i, k).unwrap();
                        next.add_edge(i, j, *direction);
                    }
                }
            }
        }

        FloydWarshallResult::new(path, next)
    }

    /// Are elements unique (no duplicates present).
    fn unique<T: Ord>(mut elements: Vec<T>) -> bool {
        let length = elements.len();

        Self::dedup(&mut elements);

        elements.len() == length
    }

    /// De-duplicate vector elements.
    fn dedup<T: Ord>(elements: &mut Vec<T>) {
        // `sort_unstable` may not preserve the order of equal elements, but it is faster and less
        // memory consuming algorithm.
        elements.sort();
        elements.dedup();
    }
}

#[cfg(test)]
mod tests {
    use crate::floyd_warshall::FloydWarshall;
    use std::cmp::Ordering::{Greater, Less};

    #[test]
    fn new() {
        let alg: FloydWarshall<f32> = FloydWarshall::new();
    }

    #[test]
    fn new_customized() {
        let mul = Box::new(|x: f32, y: f32| x + y);
        let sharp_greater = Box::new(|x: f32, y: f32| x.partial_cmp(&y).unwrap_or(Less) == Greater);

        let alg: FloydWarshall<f32> = FloydWarshall::new_customized(mul, sharp_greater);
    }
}
