//! Exchange Rate Path (ERP) algorithm.

use crate::floyd_warshall::result::FloydWarshallResult;
use crate::floyd_warshall::FloydWarshall;
use crate::graph::Graph;
use crate::request::Request;
use crate::response::best_rate_path::BestRatePath;
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
///
/// # `Algorithm` is parameterized over:
///
/// - Index `I` for indexing of nodes `N`.
/// - Node data `N`.
/// - Edge weight `E`.
pub struct Algorithm<N, E, I>
where
    N: Clone + Ord + FromStr + Eq + Hash,
    <N as FromStr>::Err: Debug,
    E: Clone + Copy + Num + PartialOrd + FromStr,
    <E as FromStr>::Err: Debug,
    I: Clone + Copy + Num + Ord + FromStr + AddAssign + Eq + Hash,
    <I as FromStr>::Err: Debug,
{
    graph: Graph<(I, I), E>,
    node_to_index: HashMap<N, I>,
    index_to_node: HashMap<I, N>,
    counter: I,
    currency_exchanges: HashMap<I, IndexSet<I>>,
}

impl<N, E, I> Algorithm<N, E, I>
where
    N: Clone + Ord + FromStr + Eq + Hash + Debug,
    <N as FromStr>::Err: Debug,
    E: Clone + Copy + Num + PartialOrd + FromStr + Debug,
    <E as FromStr>::Err: Debug,
    I: Clone + Copy + Num + Ord + FromStr + AddAssign + Eq + Hash + Debug,
    <I as FromStr>::Err: Debug,
{
    fn new() -> Self {
        let graph = Graph::<(I, I), E>::new();
        let node_to_index = HashMap::<N, I>::new();
        let index_to_node = HashMap::<I, N>::new();
        let counter = I::zero();
        let currency_exchanges = HashMap::<I, IndexSet<I>>::new();

        Self {
            graph,
            node_to_index,
            index_to_node,
            counter,
            currency_exchanges,
        }
    }

    pub fn process(request: &Request<N, E>) -> Response<(N, N), E> {
        let mut alg = Algorithm::<N, E, I>::new();
        alg.construct_graph(request);
        let result = alg.run_customized_floyd_warshall();
        alg.form_response(request, &result)
    }

    fn construct_graph(&mut self, request: &Request<N, E>) {
        // Process all `PriceUpdates`.
        for (_, price_update) in request.get_price_updates().iter() {
            // Prepare indexes.
            let exchange_index = self.node_to_index(price_update.get_exchange().clone());
            let source_currency_index =
                self.node_to_index(price_update.get_source_currency().clone());
            let destination_currency_index =
                self.node_to_index(price_update.get_destination_currency().clone());

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

    fn collect_currency_exchanges(&mut self, currency: I, exchange: I) {
        match self.currency_exchanges.entry(currency) {
            // Return the index for existing entry.
            Entry::Occupied(o) => {
                // Insert the provided exchange.
                o.into_mut().insert(exchange);
            }
            // Insert a new `IndexSet`.
            Entry::Vacant(v) => {
                // Prepare a new `IndexSet` with the provided exchange.
                let mut exchanges = IndexSet::<I>::with_capacity(1);
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

    /// Get index of the provided node `N`.
    ///
    /// If the `N` is not yet indexed, do so and return the new index.
    fn node_to_index(&mut self, s: N) -> I {
        match self.node_to_index.entry(s.clone()) {
            // Return the index for existing entry.
            Entry::Occupied(o) => *o.get(),
            // Insert with a proper index based on counter.
            Entry::Vacant(v) => {
                // Increase the counter here because new index was requested.
                self.counter += I::one();
                // Use counter as a new index.
                v.insert(self.counter);
                // Update the reverse `HashMap`.
                self.index_to_node.insert(self.counter, s);
                // Return the index.
                self.counter
            }
        }
    }

    /// Get node `N` for the provided index.
    ///
    /// Return `Option<n>` as it is possible that there's no `N` with the index.
    fn index_to_node(&self, i: &I) -> Option<&N> {
        self.index_to_node.get(i)
    }

    fn run_customized_floyd_warshall(&mut self) -> FloydWarshallResult<(I, I), E> {
        let mul = Box::new(|x: E, y: E| x * y);
        let sharp_greater = Box::new(|x: E, y: E| x.partial_cmp(&y).unwrap_or(Less) == Greater);

        let alg: FloydWarshall<E> = FloydWarshall::new_customized(mul, sharp_greater);
        let result = alg.find_paths(&self.graph);

        result
    }

    fn form_response(
        &mut self,
        request: &Request<N, E>,
        fw_result: &FloydWarshallResult<(I, I), E>,
    ) -> Response<(N, N), E> {
        let mut response = Response::new();

        // Process all `PriceUpdates`.
        for (_, rate_request) in request.get_rate_requests().iter() {
            // Prepare indexes.
            let source_exchange_index =
                self.node_to_index(rate_request.get_source_currency().clone());
            let source_currency_index =
                self.node_to_index(rate_request.get_source_currency().clone());
            let destination_exchange_index =
                self.node_to_index(rate_request.get_destination_currency().clone());
            let destination_currency_index =
                self.node_to_index(rate_request.get_destination_currency().clone());

            // Get star and end node.
            let a = (source_exchange_index, source_currency_index);
            let b = (destination_exchange_index, destination_currency_index);

            // Prepare `BestRatePath`.
            let rate_raw = fw_result.get_path_rate(a, b);
            let path = fw_result.collect_path_nodes(a, b);

            // Re-map path from indexes `I` to nodes `N`.
            let path = path
                .into_iter()
                .map(|(a, b)| {
                    (
                        self.index_to_node(&a).unwrap().clone(),
                        self.index_to_node(&b).unwrap().clone(),
                    )
                })
                .collect();

            match rate_raw {
                Some(&rate) => {
                    let best_rate_path = BestRatePath::<(N, N), E>::new(rate, path);
                    response.add_best_rate_path(best_rate_path);
                }
                None => {
                    // It would be probably good to include information about non-existing
                    // Rate request as a part of `Response` or at least log it.
                }
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use crate::algorithm::Algorithm;
    use crate::request::Request;
    use crate::response::Response;
    use std::io::BufReader;

    #[test]
    fn new() {
        let _alg = Algorithm::<String, f32, u32>::new();
    }

    #[test]
    fn collect_currency_exchanges() {
        let mut alg = Algorithm::<String, f32, u32>::new();

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
        let mut alg = Algorithm::<String, f32, u32>::new();

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
        let mut alg = Algorithm::<String, f32, u32>::new();

        let text_input = "2017-11-01T09:42:23+00:00 E1 BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 E1 ETH USD 100.0 0.001
2018-11-01T09:42:23+00:00 E2 ETH USD 100.0 0.001
2018-11-01T09:42:23+00:00 E3 ETH BTC 100.0 0.001"
            .as_bytes();

        // Test creation of Request from multiline text.
        let mut input = BufReader::new(text_input);
        let request = Request::<String, f32>::read_from(&mut input);

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
        let e1_index = alg.node_to_index(e1.clone());
        let e2_index = alg.node_to_index(e2.clone());
        let e3_index = alg.node_to_index(e3.clone());
        let btc_index = alg.node_to_index(btc.clone());
        let eth_index = alg.node_to_index(eth.clone());
        let usd_index = alg.node_to_index(usd.clone());

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
    fn run_customized_floyd_warshall() {
        let mut alg = Algorithm::<String, f32, u32>::new();

        let text_input = "2017-11-01T09:42:23+00:00 E1 BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 E1 ETH USD 102.0 0.009
2018-11-01T09:42:23+00:00 E2 ETH USD 100.0 0.0096
2018-11-01T09:42:23+00:00 E3 ETH BTC 0.08 10"
            .as_bytes();

        // Test creation of Request from multiline text.
        let mut input = BufReader::new(text_input);
        let request = Request::<String, f32>::read_from(&mut input);

        alg.construct_graph(&request);
        let result = alg.run_customized_floyd_warshall();

        // Exchanges.
        let e1 = String::from("E1");
        let e2 = String::from("E2");
        let e3 = String::from("E3");

        // Currencies.
        let btc = String::from("BTC");
        let eth = String::from("ETH");
        let usd = String::from("USD");

        //
        let e1_index = alg.node_to_index(e1.clone());
        let e2_index = alg.node_to_index(e2.clone());
        let e3_index = alg.node_to_index(e3.clone());
        let btc_index = alg.node_to_index(btc.clone());
        let eth_index = alg.node_to_index(eth.clone());
        let usd_index = alg.node_to_index(usd.clone());

        // Test rate and path from `(E1, ETH)` to `(E2, ETH)`.
        assert_eq!(
            result.get_path_rate((e1_index, eth_index), (e2_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            result.collect_path_nodes((e1_index, eth_index), (e2_index, eth_index)),
            vec![(e1_index, eth_index), (e2_index, eth_index)]
        );

        // Test rate and path from `(E1, ETH)` to `(E3, ETH)`.
        assert_eq!(
            result.get_path_rate((e1_index, eth_index), (e3_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            result.collect_path_nodes((e1_index, eth_index), (e3_index, eth_index)),
            vec![(e1_index, eth_index), (e3_index, eth_index)]
        );

        // Test rate and path from `(E2, ETH)` to `(E3, ETH)`.
        assert_eq!(
            result.get_path_rate((e2_index, eth_index), (e3_index, eth_index)),
            Some(&1.0)
        );
        assert_eq!(
            result.collect_path_nodes((e2_index, eth_index), (e3_index, eth_index)),
            vec![(e2_index, eth_index), (e3_index, eth_index)]
        );

        // Test rate and path from `(E2, ETH)` to `(E1, USD)`.
        assert_eq!(
            result.get_path_rate((e2_index, eth_index), (e1_index, usd_index)),
            Some(&102.0)
        );
        assert_eq!(
            result.collect_path_nodes((e2_index, eth_index), (e1_index, usd_index)),
            vec![
                (e2_index, eth_index),
                (e1_index, eth_index),
                (e1_index, usd_index)
            ]
        );
    }

    #[test]
    fn process() {
        let text_input = "2017-11-01T09:42:23+00:00 E1 BTC USD 1000.0 0.0009
2018-11-01T09:42:23+00:00 E1 ETH USD 100.0 0.01
2018-11-01T09:42:23+00:00 E2 ETH USD 100.0 0.01
2018-11-01T09:42:23+00:00 E2 BTC USD 1002.0 0.00092
2018-11-01T09:42:23+00:00 E3 ETH BTC 100.0 0.01
EXCHANGE_RATE_REQUEST E1 BTC E3 ETH
EXCHANGE_RATE_REQUEST E1 BTC E3 USD"
            .as_bytes();

        // Test creation of Request from multiline text.
        let mut input = BufReader::new(text_input);
        let request = Request::<String, f32>::read_from(&mut input);

        let response = Algorithm::<String, f32, u32>::process(&request);

        // println!("{}", response.get_best_rate_path());

        // assert_eq!(response.get_best_rate_path().first().unwrap().get_rate(), &1.2);
    }
}
