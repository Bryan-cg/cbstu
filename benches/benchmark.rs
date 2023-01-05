use criterion::{black_box, criterion_group, criterion_main, Criterion};
use final_network_sts::io::input_handler::InputHandler;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use final_network_sts::datastructures::graph::immutable_graph::ImmutableGraph;

fn edge_elimination(graph: &ImmutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = EdgeElimination::run(graph, budget);
    bottleneck
}

fn criterion_benchmark(c: &mut Criterion) {
    let graph = InputHandler::read("test_data/ta2--D-B-E-N-C-A-N-N_network65_108.json");
    let neg_graph = graph.negative_weights();
    c.bench_function("edge_elm", |b| b.iter(|| edge_elimination(black_box(&neg_graph), black_box(100.0))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);