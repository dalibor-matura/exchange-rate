//! Exchange Rate Path Response.

pub mod best_rate_path;

use self::best_rate_path::BestRatePath;
use crate::floyd_warshall::FloydWarshallTrait;
use std::fmt::{Debug, Display};

/// Exchange Rate Path `Response` structure.
///
/// # `Response<N, E>` is parameterized over:
///
/// - Identifier data `N`.
/// - Edge weight `E`.
pub struct Response<N, E> {
    best_rate_paths: Vec<BestRatePath<N, E>>,
}

impl<N, E> Response<N, E>
where
    N: Display + Debug,
    E: FloydWarshallTrait + Display,
{
    pub fn new() -> Self {
        Self {
            best_rate_paths: Vec::new(),
        }
    }

    pub fn add_best_rate_path(&mut self, best_rate_path: BestRatePath<N, E>) {
        self.best_rate_paths.push(best_rate_path);
    }

    #[allow(dead_code)]
    pub fn get_best_rate_path(&self) -> &Vec<BestRatePath<N, E>> {
        &self.best_rate_paths
    }

    /// Get printable output representing the Response.
    ///
    /// Concatenate all outputs of `BestRatePath`s.
    pub fn get_output(&self) -> String {
        let mut output = String::new();

        for best_rate_path in self.best_rate_paths.iter() {
            output.push_str(&best_rate_path.get_output());
        }

        output
    }
}

#[cfg(test)]
mod tests {}
