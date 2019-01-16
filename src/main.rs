use exchange_rate_path::graph::DiGraphMap;

fn main() {
    let mut graph: DiGraphMap<&str, f32> = DiGraphMap::with_capacity(4, 6);
    graph.add_edge("a", "b", 2.0);

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
    assert!(graph.contains_edge("a", "b"));
    assert!(!graph.contains_edge("b", "a"));

    graph.add_edge("a", "c", 1.2);
    graph.add_edge("a", "d", 4.2);
    graph.add_edge("b", "c", 0.2);
    graph.add_edge("b", "d", 3.3);
    graph.add_edge("c", "b", 12.2);

    // Check numbers of nodes and edges.
    assert_eq!(graph.node_count(), 4);
    assert_eq!(graph.edge_count(), 6);

    // Check edges weight.
    assert_eq!(graph.edge_weight("a", "b"), Some(&2.0));
    assert_eq!(graph.edge_weight("a", "c"), Some(&1.2));

    // Update and check edge weight.
    graph.add_edge("a", "b", 4.4);

    assert_eq!(graph.edge_weight("a", "b"), Some(&4.4));

    // Try to get edge weight for non-existing edge.
    let weight = graph.edge_weight("c", "d");

    assert_eq!(weight, None);

    // Get some output.

    println!("Directed Graph: {:?}", graph);
}
