//! Exchange Rate Path (ERP) algorithm.

use crate::graph::Graph;
use crate::request::Request;
use crate::response::Response;
use indexmap::IndexSet;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialOrd};
use std::collections::hash_map::{Entry, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;
use std::str::FromStr;

/// Exchange Rate Path (ERP) Algorithm structure.
pub struct Algorithm<N>
where
    N: Clone + Copy + Num + Ord + FromStr + AddAssign + Eq + Hash,
    <N as FromStr>::Err: Debug,
{
    string_to_index: HashMap<String, N>,
    index_to_string: HashMap<N, String>,
    counter: N,
    currency_exchanges: HashMap<N, IndexSet<N>>,
}

impl<N> Algorithm<N>
where
    N: Clone + Copy + Num + Ord + FromStr + AddAssign + Eq + Hash,
    <N as FromStr>::Err: Debug,
{
    fn new() -> Self {
        let string_to_index = HashMap::<String, N>::new();
        let index_to_string = HashMap::<N, String>::new();
        let counter = N::zero();
        let currency_exchanges = HashMap::<N, IndexSet<N>>::new();

        Self {
            string_to_index,
            index_to_string,
            counter,
            currency_exchanges,
        }
    }

    pub fn process<T>(request: &Request<T>) -> Response
    where
        T: Clone + Copy + Num + PartialOrd + FromStr,
        <T as FromStr>::Err: Debug,
    {
        let mut alg = Algorithm::<N>::new();
        let graph = alg.construct_graph(request);

        Response {}
    }

    fn construct_graph<E>(&mut self, request: &Request<E>) -> Graph<(N, N), E>
    where
        E: Clone + Copy + Num + PartialOrd + FromStr,
        <E as FromStr>::Err: Debug,
    {
        let mut graph = Graph::<(N, N), E>::new();

        // Process all `PriceUpdates`.
        for (_, price_update) in request.get_price_updates().iter() {
            // Prepare indexes.
            let exchange_index = self.string_to_index(price_update.get_exchange().clone());
            let source_currency_index =
                self.string_to_index(price_update.get_source_currency().clone());
            let destination_currency_index =
                self.string_to_index(price_update.get_destination_currency().clone());

            // Get star and end node.
            let a = (exchange_index, source_currency_index);
            let b = (exchange_index, destination_currency_index);

            // Add forward edge.
            graph.add_edge(a, b, *price_update.get_forward_factor());
            // Add backward edge.
            graph.add_edge(b, a, *price_update.get_backward_factor());

            // Collect provided currencies.
            self.collect_currency_exchanges(source_currency_index, exchange_index);
            self.collect_currency_exchanges(destination_currency_index, exchange_index);
        }

        // For each currency add edges, so that each `(exchange, currency)` is connected to every
        // other `(other_exchange, currency)` with an edge weight of 1.0.
        self.add_currency_exchanges_edges(&mut graph);

        graph
    }

    fn collect_currency_exchanges(&mut self, currency: N, exchange: N) {
        match self.currency_exchanges.entry(currency) {
            // Return the index for existing entry.
            Entry::Occupied(o) => {
                // Insert the provided exchange.
                o.into_mut().insert(exchange);
            }
            // Insert a new `IndexSet`.
            Entry::Vacant(v) => {
                // Prepare a new `IndexSet` with the provided exchange.
                let mut exchanges = IndexSet::<N>::with_capacity(1);
                exchanges.insert(exchange);

                // Insert the new `IndexSet`.
                v.insert(exchanges);
            }
        }
    }

    fn add_currency_exchanges_edges<E>(&self, graph: &mut Graph<(N, N), E>)
    where
        E: Clone + Copy + Num + PartialOrd + FromStr,
        <E as FromStr>::Err: Debug,
    {
        // Loop through all currencies.
        for (currency, exchanges) in self.currency_exchanges.iter() {
            let exchanges_count = exchanges.len();

            // Loop through exchanges of the current currency.
            for top in 0..exchanges_count {
                // Loop through all exchanges of the current currency following the previous
                // top exchange.
                for below in top + 1..exchanges_count {
                    let a = (*exchanges.get_index(top).unwrap(), *currency);
                    let b = (*exchanges.get_index(below).unwrap(), *currency);

                    // Add forward edge.
                    graph.add_edge(a, b, E::one());
                    // Add backward edge.
                    graph.add_edge(b, a, E::one());
                }
            }
        }
    }

    /// Get index of the provided `String`.
    ///
    /// If the `String` is not yet indexed, do so and return the new index.
    fn string_to_index(&mut self, s: String) -> N {
        match self.string_to_index.entry(s.clone()) {
            // Return the index for existing entry.
            Entry::Occupied(o) => *o.get(),
            // Insert with a proper index based on counter.
            Entry::Vacant(v) => {
                // Increase the counter here because new index was requested.
                self.counter += N::one();
                // Use counter as a new index.
                *v.insert(self.counter);
                // Update the reverse `HashMap`.
                self.index_to_string.insert(self.counter, s);
                // Return the index.
                self.counter
            }
        }
    }

    /// Get `String` for the provided index.
    ///
    /// Return `Option<String>` as it is possible that there's no `String` with the index.
    fn index_to_string(&self, i: &N) -> Option<&String> {
        self.index_to_string.get(i)
    }
}

#[cfg(test)]
mod tests {}
