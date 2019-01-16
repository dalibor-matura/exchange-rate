//! Directed Graph representation.
//!
//! The Graph and its components are inspired and mostly copied and refactored from `petgraph` crate
//! https://crates.io/crates/petgraph.
//!
//! This project requires just some parts of the `petgraph` like: NodeIndex, EdgeIndex and Graph.
//! Additionally this project needs to do some modification to implementation of `petgraph`.
//!
//! But the most problematic part woudl be a security implication of using a third-party crate with
//! no stable release yet, risking of malicious actions by outside influence.
//!
//! # Examples
//!
//! ```
//! ```

use crate::graph::edge::{Edge, EdgeIndex};
use crate::graph::node::{Node, NodeIndex, NodeIndices};
use crate::graph::Direction::{Incoming, Outgoing};
use std::fmt;
use std::hash::Hash;
use std::iter;
use std::slice;

#[macro_use]
mod macros;
pub mod edge;
pub mod node;

/// The default integer type for graph indices.
/// `u32` is the default to reduce the size of the graph's data and improve
/// performance in the common case.
///
/// Used for node and edge indices in `Graph` and `StableGraph`, used
/// for node indices in `Csr`.
pub type DefaultIx = u32;

/// Trait for the unsigned integer type used for node and edge indices.
///
/// Marked `unsafe` because: the trait must faithfully preseve
/// and convert index values.
pub unsafe trait IndexType: Copy + Default + Hash + Ord + fmt::Debug + 'static {
    fn new(x: usize) -> Self;
    fn index(&self) -> usize;
    fn max() -> Self;
}

unsafe impl IndexType for usize {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x
    }
    #[inline(always)]
    fn index(&self) -> Self {
        *self
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::usize::MAX
    }
}

unsafe impl IndexType for u32 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u32
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u32::MAX
    }
}

unsafe impl IndexType for u16 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u16
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u16::MAX
    }
}

unsafe impl IndexType for u8 {
    #[inline(always)]
    fn new(x: usize) -> Self {
        x as u8
    }
    #[inline(always)]
    fn index(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    fn max() -> Self {
        ::std::u8::MAX
    }
}

// Index into the NodeIndex and EdgeIndex arrays
/// Edge direction.
#[derive(Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    /// An `Outgoing` edge is an outward edge *from* the current node.
    Outgoing = 0,
    /// An `Incoming` edge is an inbound edge *to* the current node.
    Incoming = 1,
}

copyclone!(Direction);

impl Direction {
    /// Return the opposite `Direction`.
    #[inline]
    pub fn opposite(&self) -> Direction {
        match *self {
            Outgoing => Incoming,
            Incoming => Outgoing,
        }
    }

    /// Return `0` for `Outgoing` and `1` for `Incoming`.
    #[inline]
    pub fn index(&self) -> usize {
        (*self as usize) & 0x1
    }
}

/// An iterator over either the nodes without edges to them or from them.
pub struct Externals<'a, N: 'a, Ix: IndexType = DefaultIx> {
    iter: iter::Enumerate<slice::Iter<'a, Node<N, Ix>>>,
    dir: Direction,
}

impl<'a, N: 'a, Ix: IndexType> Externals<'a, N, Ix> {
    pub fn new(iter: iter::Enumerate<slice::Iter<'a, Node<N, Ix>>>, dir: Direction) -> Self {
        Self { iter, dir }
    }
}

impl<'a, N: 'a, Ix: IndexType> Iterator for Externals<'a, N, Ix> {
    type Item = NodeIndex<Ix>;

    fn next(&mut self) -> Option<NodeIndex<Ix>> {
        let k = self.dir.index();
        loop {
            match self.iter.next() {
                None => return None,
                Some((index, node)) => {
                    if node.next[k] == EdgeIndex::end() {
                        return Some(NodeIndex::new(index));
                    } else {
                        continue;
                    }
                }
            }
        }
    }
}

/// `Graph<N, E, Ix>` is a graph datastructure using an adjacency list representation.
///
/// `Graph` is parameterized over:
///
/// - Associated names `N` for nodes.
///   The associated data can be of arbitrary type.
/// - Associated data `E` for edges, called *weights*.
///   The associated data can be of arbitrary type.
/// - Index type `Ix`, which determines the maximum size of the graph.
///
/// The graph uses **O(|V| + |E|)** space, and allows fast node and edge insert,
/// efficient graph search and graph algorithms.
/// It implements **O(e')** edge lookup , where **e'** is some local measure of edge count.
/// Based on the graph datastructure used in rustc.
///
/// ### Graph Indices
///
/// The graph maintains indices for nodes and edges, and edge weights may be accessed mutably.
/// Indices range in a compact interval, for example for *n* nodes indices are 0 to *n* - 1
/// inclusive.
///
/// `NodeIndex` and `EdgeIndex` are types that act as references to nodes and edges,
/// but these are only stable across certain operations.
/// **Adding nodes or edges keeps indices stable.
/// Removing nodes or edges may shift other indices.**
///
/// The `Ix` parameter is `u32` by default. The goal is that you can ignore this parameter
/// completely unless you need a very big graph -- then you can use `usize`.
///
/// ### Pros and Cons of Indices
///
/// * The fact that the node and edge indices in the graph each are numbered in compact
/// intervals (from 0 to *n* - 1 for *n* nodes) simplifies some graph algorithms.
///
/// * You can select graph index integer type after the size of the graph. A smaller
/// size may have better performance.
///
/// * Using indices allows mutation while traversing the graph, see `Dfs`,
/// and `.neighbors(a).detach()`.
///
/// * The `Graph` is a regular rust collection and is `Send` and `Sync` (as long
/// as associated data `N` and `E` are).
pub struct Graph<N, E, Ix: IndexType = DefaultIx> {
    nodes: Vec<Node<N, Ix>>,
    edges: Vec<Edge<E, Ix>>,
}

impl<N, E, Ix> Graph<N, E, Ix>
where
    Ix: IndexType,
{
    /// Create a new `Graph`.
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Create a new `Graph` with estimated capacity.
    pub fn with_capacity(nodes: usize, edges: usize) -> Self {
        Graph {
            nodes: Vec::with_capacity(nodes),
            edges: Vec::with_capacity(edges),
        }
    }

    /// Return the number of nodes (vertices) in the graph.
    ///
    /// Computes in **O(1)** time.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Return the number of edges in the graph.
    ///
    /// Computes in **O(1)** time.
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    /// Add a node (also called vertex) with associated data `weight` to the graph.
    ///
    /// Computes in **O(1)** time.
    ///
    /// Return the index of the new node.
    ///
    /// # Panics
    ///
    /// Panics if the Graph is at the maximum number of nodes for its index
    /// type (N/A if usize).
    pub fn add_node(&mut self, name: N) -> NodeIndex<Ix> {
        let node = Node::new(name);
        let node_idx = NodeIndex::new(self.nodes.len());

        // Check for max capacity, except if we use usize.
        assert!(<Ix as IndexType>::max().index() == !0 || NodeIndex::end() != node_idx);

        self.nodes.push(node);
        node_idx
    }

    /// Return an iterator over either the nodes without edges to them
    /// (`Incoming`) or from them (`Outgoing`).
    ///
    /// An *internal* node has both incoming and outgoing edges.
    /// The nodes in `.externals(Incoming)` are the source nodes and
    /// `.externals(Outgoing)` are the sinks of the graph.
    ///
    /// For a graph with undirected edges, both the sinks and the sources are
    /// just the nodes without edges.
    ///
    /// The whole iteration computes in **O(|V|)** time.
    pub fn externals(&self, dir: Direction) -> Externals<N, Ix> {
        Externals::new(self.nodes.iter().enumerate(), dir)
    }

    /// Return an iterator over the node indices of the graph
    pub fn node_indices(&self) -> NodeIndices<Ix> {
        NodeIndices::new(0..self.node_count())
    }
}

#[cfg(test)]
mod tests {}
