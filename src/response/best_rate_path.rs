//! Best Rate Path.

use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt;
use std::hash::Hash;

pub struct BestRatePath<N, E>
where
    N: Eq + Copy + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    rate: E,
    path: Vec<N>,
}

impl<N, E> BestRatePath<N, E>
where
    N: Eq + Copy + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
}
