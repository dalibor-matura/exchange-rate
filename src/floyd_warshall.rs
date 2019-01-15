//! Floyd-Warshall algorithm.

use crate::graph::DirectedGraph;

/// Floyd-Warshall algorithm structure.
///
/// # Examples
/// ```
/// ```
pub struct FloydWarshall<T: DirectedGraph> {
    graph: Box<T>,
}

#[cfg(test)]
mod tests {}
