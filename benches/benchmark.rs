use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use final_network_sts::io::input_handler::InputHandler;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use final_network_sts::datastructures::graph::immutable_graph::ImmutableGraph;

fn edge_elimination(graph: &ImmutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = EdgeElimination::run(graph, budget);
    bottleneck
}

fn punnen(graph: &ImmutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = Punnen::run(graph, budget);
    bottleneck
}

fn berman(graph: &ImmutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = Berman::run(graph, budget);
    bottleneck
}

fn criterion_benchmark_algorithms(c: &mut Criterion) {
    let graph = InputHandler::read("data/wrp4-76_network766_1535.json");
    let neg_graph = graph.negative_weights();
    let mut group = c.benchmark_group("Algorithms");
    group.measurement_time(Duration::from_secs(8));
    group.bench_function("punnen", |b| b.iter(|| punnen(black_box(&neg_graph), black_box(1000.0))));
    group.bench_function("berman", |b| b.iter(|| berman(black_box(&neg_graph), black_box(1000.0))));
    group.bench_function("edge_elm", |b| b.iter(|| edge_elimination(black_box(&neg_graph), black_box(1000.0))));
    group.finish();
}

criterion_group!(benches, criterion_benchmark_algorithms);
criterion_main!(benches);