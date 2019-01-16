//! Graph Node representation.
//!
//! # Examples
//!
//! ```
//! ```

use crate::graph::edge::EdgeIndex;
use crate::graph::{DefaultIx, Direction, IndexType};
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Range};

/// Node identifier.
#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeIndex<Ix = DefaultIx>(Ix);

impl<Ix: IndexType> NodeIndex<Ix> {
    #[inline]
    pub fn new(x: usize) -> Self {
        NodeIndex(IndexType::new(x))
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0.index()
    }

    #[inline]
    pub fn end() -> Self {
        NodeIndex(IndexType::max())
    }
}

impl<Ix: IndexType> From<Ix> for NodeIndex<Ix> {
    fn from(ix: Ix) -> Self {
        NodeIndex(ix)
    }
}

impl<Ix: fmt::Debug> fmt::Debug for NodeIndex<Ix> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NodeIndex({:?})", self.0)
    }
}

/// Short version of `NodeIndex::new`
pub fn node_index<Ix: IndexType>(index: usize) -> NodeIndex<Ix> {
    NodeIndex::new(index)
}

/// The graph's node type.
#[derive(Debug)]
pub struct Node<N, Ix = DefaultIx> {
    /// Associated node name.
    pub name: N,
    /// Next edge in outgoing and incoming edge lists.
    pub(super) next: [EdgeIndex<Ix>; 2],
}

impl<N, Ix> Clone for Node<N, Ix>
where
    N: Clone,
    Ix: Copy,
{
    clone_fields!(Node, name, next);
}

impl<N, Ix: IndexType> Node<N, Ix> {
    /// Accessor for data structure internals: the first edge in the given direction.
    pub fn next_edge(&self, dir: Direction) -> EdgeIndex<Ix> {
        self.next[dir.index()]
    }

    pub fn new(name: N) -> Self {
        Self {
            name,
            next: [EdgeIndex::end(), EdgeIndex::end()],
        }
    }
}

/// Iterator over the node indices of a graph.
#[derive(Clone, Debug)]
pub struct NodeIndices<Ix: IndexType = DefaultIx> {
    r: Range<usize>,
    ty: PhantomData<fn() -> Ix>,
}

impl<Ix: IndexType> NodeIndices<Ix> {
    pub fn new(range: Range<usize>) -> Self {
        Self {
            r: range,
            ty: PhantomData,
        }
    }
}

impl<Ix: IndexType> Iterator for NodeIndices<Ix> {
    type Item = NodeIndex<Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        self.r.next().map(node_index)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.r.size_hint()
    }
}

impl<Ix: IndexType> DoubleEndedIterator for NodeIndices<Ix> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.r.next_back().map(node_index)
    }
}

impl<Ix: IndexType> ExactSizeIterator for NodeIndices<Ix> {}
