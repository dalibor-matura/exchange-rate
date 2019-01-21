//! Exchange Rate Path Response.

pub mod best_rate_path;

use self::best_rate_path::BestRatePath;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::PartialOrd;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;
use std::str::FromStr;

/// Exchange Rate Path Response structure.
pub struct Response<N, E>
where
    N: Display + Eq + FromStr + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    best_rate_paths: Vec<BestRatePath<N, E>>,
}

impl<N, E> Response<N, E>
where
    N: Display + Eq + FromStr + Hash + Ord + fmt::Debug,
    E: Clone + Copy + Num + PartialOrd,
{
    pub fn new() -> Self {
        Self {
            best_rate_paths: Vec::new(),
        }
    }

    pub fn add_best_rate_path(&mut self, best_rate_path: BestRatePath<N, E>) {
        self.best_rate_paths.push(best_rate_path);
    }

    pub fn get_best_rate_path(&self) -> &Vec<BestRatePath<N, E>> {
        &self.best_rate_paths
    }
}

#[cfg(test)]
mod tests {}
