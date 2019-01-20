//! Exchange Rate Path Response.

mod best_rate_path;

use self::best_rate_path::BestRatePath;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt;
use std::hash::Hash;

/// Exchange Rate Path Response structure.
pub struct Response<N, E>
where
    N: Eq + Copy + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    best_rate_paths: Vec<BestRatePath<N, E>>,
}

impl<N, E> Response<N, E>
where
    N: Eq + Copy + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            best_rate_paths: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {}
