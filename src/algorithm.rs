//! Exchange Rate Path (ERP) algorithm.

use crate::floyd_warshall::result::FloydWarshallResult;
use crate::floyd_warshall::FloydWarshall;
use crate::graph::Graph;
use crate::request::Request;
use crate::response::Response;
use indexmap::IndexSet;
use num_traits::Num;
use std::clone::Clone;
use std::cmp::Ordering::{Greater, Less};
use std::cmp::{Eq, Ord, PartialOrd};
use std::collections::hash_map::{Entry, HashMap};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;
use std::str::FromStr;

/// Exchange Rate Path (ERP) Algorithm structure.
pub struct Algorithm<N, E>
where
    N: Clone + Copy + Num + Ord + FromStr + AddAssign + Eq + Hash,
    <N as FromStr>::Err: Debug,
    E: Clone + Copy + Num + PartialOrd + FromStr,
    <E as FromStr>::Err: Debug,
{
    graph: Graph<(N, N), E>,
    string_to_index: HashMap<String, N>,
    index_to_string: HashMap<N, String>,
    counter: N,
    currency_exchanges: HashMap<N, IndexSet<N>>,
}

impl<N, E> Algorithm<N, E>
where
    N: Clone + Copy + Num + Ord + FromStr + AddAssign + Eq + Hash + Debug,
    <N as FromStr>::Err: Debug,
    E: Clone + Copy + Num + PartialOrd + FromStr,
    <E as FromStr>::Err: Debug,
{
    fn new() -> Self {
        let graph = Graph::<(N, N), E>::new();
        let string_to_index = HashMap::<String, N>::new();
        let index_to_string = HashMap::<N, String>::new();
        let counter = N::zero();
        let currency_exchanges = HashMap::<N, IndexSet<N>>::new();

        Self {
            graph,
            string_to_index,
            index_to_string,
            counter,
            currency_exchanges,
        }
    }

    pub fn process(request: &Request<E>) -> Response {
        let mut alg = Algorithm::<N, E>::new();
        alg.construct_graph(request);
        let result = alg.run_customized_floyd_warshall();

        Response {}
    }

    fn construct_graph(&mut self, request: &Request<E>) {
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
            self.graph
                .add_edge(a, b, *price_update.get_forward_factor());
            // Add backward edge.
            self.graph
                .add_edge(b, a, *price_update.get_backward_factor());

            // Collect provided currencies.
            self.collect_currency_exchanges(source_currency_index, exchange_index);
            self.collect_currency_exchanges(destination_currency_index, exchange_index);
        }

        // For each currency add edges, so that each `(exchange, currency)` is connected to every
        // other `(other_exchange, currency)` with an edge weight of 1.0.
        self.add_currency_exchanges_edges();
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

    fn add_currency_exchanges_edges(&mut self) {
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
                    self.graph.add_edge(a, b, E::one());
                    // Add backward edge.
                    self.graph.add_edge(b, a, E::one());
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
                v.insert(self.counter);
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

    fn run_customized_floyd_warshall(&mut self) -> FloydWarshallResult<(N, N), E> {
        let mul = Box::new(|x: E, y: E| x + y);
        let sharp_greater = Box::new(|x: E, y: E| x.partial_cmp(&y).unwrap_or(Less) == Greater);

        let alg: FloydWarshall<E> = FloydWarshall::new_customized(mul, sharp_greater);
        let result = alg.find_paths(&self.graph);

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::Algorithm;
    use crate::request::Request;
    use std::io::BufReader;

    #[test]
    fn new() {
        let _alg = Algorithm::<u32, f32>::new();
    }

    #[test]
    fn collect_currency_exchanges() {
        let mut alg = Algorithm::<u32, f32>::new();

        alg.collect_currency_exchanges(1, 2);
        alg.collect_currency_exchanges(1, 3);
        alg.collect_currency_exchanges(1, 4);
        alg.collect_currency_exchanges(5, 6);

        // Test currencies existence.
        assert!(alg.currency_exchanges.get(&1).is_some());
        assert!(alg.currency_exchanges.get(&5).is_some());

        // Test currencies non-existence.
        assert!(alg.currency_exchanges.get(&2).is_none());

        // Test exchanges existence.
        assert!(alg.currency_exchanges.get(&1).unwrap().get(&2).is_some());
        assert_eq!(alg.currency_exchanges.get(&1).unwrap().get(&2).unwrap(), &2);
        assert!(alg.currency_exchanges.get(&1).unwrap().get(&3).is_some());
        assert_eq!(alg.currency_exchanges.get(&1).unwrap().get(&3).unwrap(), &3);
        assert!(alg.currency_exchanges.get(&5).unwrap().get(&6).is_some());
        assert_eq!(alg.currency_exchanges.get(&5).unwrap().get(&6).unwrap(), &6);

        // Test exchanges non-existence.
        assert!(alg.currency_exchanges.get(&1).unwrap().get(&7).is_none());
    }

    #[test]
    fn add_currency_exchanges_edges() {
        let mut alg = Algorithm::<u32, f32>::new();

        alg.collect_currency_exchanges(1, 2);
        alg.collect_currency_exchanges(1, 3);
        alg.collect_currency_exchanges(1, 4);
        alg.collect_currency_exchanges(5, 6);

        alg.add_currency_exchanges_edges();

        // Test edges existence.
        assert_eq!(alg.graph.edge_weight((2, 1), (3, 1)), Some(&1.0));
        assert_eq!(alg.graph.edge_weight((3, 1), (2, 1)), Some(&1.0));
        assert_eq!(alg.graph.edge_weight((2, 1), (4, 1)), Some(&1.0));
        assert_eq!(alg.graph.edge_weight((4, 1), (2, 1)), Some(&1.0));
        assert_eq!(alg.graph.edge_weight((3, 1), (4, 1)), Some(&1.0));
        assert_eq!(alg.graph.edge_weight((4, 1), (3, 1)), Some(&1.0));

        // Test edges non-existence.
        assert_eq!(alg.graph.contains_edge((2, 1), (6, 1)), false);
        assert_eq!(alg.graph.contains_edge((6, 1), (2, 1)), false);
        assert_eq!(alg.graph.contains_edge((6, 5), (2, 5)), false);
    }

    #[test]
    fn construct_graph() {
        let mut alg = Algorithm::<u32, f32>::new();

        let text_input = "2017-11-01T09:42:23+00:00 E1 BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 E1 ETH USD 100.0 0.001
2018-11-01T09:42:23+00:00 E2 ETH USD 100.0 0.001
2018-11-01T09:42:23+00:00 E3 ETH BTC 100.0 0.001"
            .as_bytes();

        // Test creation of Request from multiline text.
        let mut input = BufReader::new(text_input);
        let request = Request::<f32>::read_from(&mut input);

        alg.construct_graph(&request);

        // Exchanges.
        let e1 = String::from("E1");
        let e2 = String::from("E2");
        let e3 = String::from("E3");

        // Currencies.
        let btc = String::from("BTC");
        let eth = String::from("ETH");
        let usd = String::from("USD");

        //
        let e1_index = alg.string_to_index(e1.clone());
        let e2_index = alg.string_to_index(e2.clone());
        let e3_index = alg.string_to_index(e3.clone());
        let btc_index = alg.string_to_index(btc.clone());
        let eth_index = alg.string_to_index(eth.clone());
        let usd_index = alg.string_to_index(usd.clone());

        // Test ETH edges existence.
        assert_eq!(
            alg.graph
                .edge_weight((e1_index, eth_index), (e2_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e2_index, eth_index), (e1_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e1_index, eth_index), (e3_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e3_index, eth_index), (e1_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e2_index, eth_index), (e3_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e3_index, eth_index), (e2_index, eth_index)),
            Some(&1.0)
        );

        // Test USF edges existence.
        assert_eq!(
            alg.graph
                .edge_weight((e1_index, usd_index), (e2_index, usd_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e2_index, usd_index), (e1_index, usd_index)),
            Some(&1.0)
        );

        // Test USD edges non-existence.
        assert_eq!(
            alg.graph
                .contains_edge((e1_index, usd_index), (e3_index, usd_index)),
            false
        );
        assert_eq!(
            alg.graph
                .contains_edge((e3_index, usd_index), (e1_index, usd_index)),
            false
        );
        assert_eq!(
            alg.graph
                .contains_edge((e2_index, usd_index), (e3_index, usd_index)),
            false
        );
        assert_eq!(
            alg.graph
                .contains_edge((e3_index, usd_index), (e2_index, usd_index)),
            false
        );

        // Test BTC edges existence.
        assert_eq!(
            alg.graph
                .edge_weight((e1_index, btc_index), (e3_index, btc_index)),
            Some(&1.0)
        );
        assert_eq!(
            alg.graph
                .edge_weight((e3_index, btc_index), (e1_index, btc_index)),
            Some(&1.0)
        );
    }

    #[test]
    fn process() {
        let text_input = "2017-11-01T09:42:23+00:00 E1 BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 E1 ETH USD 100.0 0.001
2018-11-01T09:42:23+00:00 E2 ETH USD 100.0 0.001
2018-11-01T09:42:23+00:00 E3 ETH BTC 100.0 0.001"
            .as_bytes();

        // Test creation of Request from multiline text.
        let mut input = BufReader::new(text_input);
        let request = Request::<f32>::read_from(&mut input);

        let _alg = Algorithm::<u32, f32>::process(&request);
    }
}
