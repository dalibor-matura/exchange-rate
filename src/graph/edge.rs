//! Graph Edge representation.
//!
//! # Examples
//!
//! ```
//! ```

use crate::graph::node::NodeIndex;
use crate::graph::{DefaultIx, Direction, IndexType};
use std::fmt;

/// Edge identifier.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EdgeIndex<Ix = DefaultIx>(Ix);

impl<Ix: IndexType> EdgeIndex<Ix> {
    #[inline]
    pub fn new(x: usize) -> Self {
        EdgeIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        EdgeIndex(IndexType::max())
    }
}

impl<Ix: fmt::Debug> fmt::Debug for EdgeIndex<Ix> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "EdgeIndex({:?})", self.0)
    }
}

/// Short version of `EdgeIndex::new`
pub fn edge_index<Ix: IndexType>(index: usize) -> EdgeIndex<Ix> {
    EdgeIndex::new(index)
}

/// The graph's edge type.
#[derive(Debug)]
pub struct Edge<E, Ix: IndexType = DefaultIx> {
    /// Associated edge data.
    weight: E,
    /// Next edge in outgoing and incoming edge lists.
    pub(super) next: [EdgeIndex<Ix>; 2],
    /// Start and End node index
    pub(super) node: [NodeIndex<Ix>; 2],
}

impl<E, Ix: IndexType> Edge<E, Ix> {
    pub fn new(a: NodeIndex<Ix>, b: NodeIndex<Ix>, weight: E) -> Self {
        Self {
            weight,
            node: [a, b],
            next: [EdgeIndex::end(); 2],
        }
    }
}

impl<E, Ix: IndexType> Clone for Edge<E, Ix>
where
    E: Clone,
    Ix: Copy,
{
    clone_fields!(Edge, weight, next, node,);
}

impl<E, Ix: IndexType> Edge<E, Ix> {
    /// Accessor for data structure internals: the next edge for the given direction.
    pub fn next_edge(&self, dir: Direction) -> EdgeIndex<Ix> {
        self.next[dir.index()]
    }

    /// Return the source node index.
    pub fn source(&self) -> NodeIndex<Ix> {
        self.node[0]
    }

    /// Return the target node index.
    pub fn target(&self) -> NodeIndex<Ix> {
        self.node[1]
    }
}
