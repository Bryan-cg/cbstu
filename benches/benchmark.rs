use std::time::Duration;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use final_network_sts::io::input_handler::InputHandler;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
use final_network_sts::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use final_network_sts::algorithms::min_bottleneck_spanning_tree::camerini::MBST;
use final_network_sts::algorithms::min_sum_spanning_tree::kruskal::CalculationType;
use final_network_sts::datastructures::graph::mutable_graph::MutableGraph;

fn edge_elimination(graph: &MutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = EdgeElimination::run(graph, budget);
    bottleneck
}

fn punnen(graph: &MutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = Punnen::run(graph, budget);
    bottleneck
}

fn berman(graph: &MutableGraph, budget: f64) -> f64 {
    let (_, _, bottleneck) = Berman::run(graph, budget);
    bottleneck
}

fn mbst(graph: &mut MutableGraph) -> f64 {
    let (_, bottleneck) = MBST::run_mutable(graph);
    bottleneck
}

fn mst(graph: &mut MutableGraph) -> f64 {
    let (_, _, bottleneck) = graph.min_sum_st(CalculationType::Weight);
    bottleneck
}

fn criterion_benchmark_algorithms(c: &mut Criterion) {
    let mut graph_giant_mut = InputHandler::read_mut("data/es10000fst01_network27019_39407.json");
    let mut graph_big_mut = InputHandler::read_mut("data/wrp4-76_network766_1535.json");
    let mut graph_mid_mut = InputHandler::read_mut("data/wrp4-11_network123_233.json");
    let mut graph_small_mut = InputHandler::read_mut("data/pioro40--D-B-M-N-C-A-N-N_network40_89.json");
    let mut graph_tiny_mut = InputHandler::read_mut("data/atlanta--D-B-M-N-C-A-N-N_network15_22.json");
    graph_giant_mut.inverse_weights();
    graph_big_mut.inverse_weights();
    graph_mid_mut.inverse_weights();
    graph_small_mut.inverse_weights();
    graph_tiny_mut.inverse_weights();
    let mut group = c.benchmark_group("Algorithms");
    group.measurement_time(Duration::from_secs(10));
    //group.bench_function("punnen_giant", |b| b.iter(|| punnen(black_box(&graph_giant), black_box(1000.0))));
    //group.bench_function("berman_giant", |b| b.iter(|| berman(black_box(&graph_giant), black_box(1000.0))));
    //group.bench_function("edge_elimination_giant", |b| b.iter(|| edge_elimination(black_box(&graph_giant), black_box(1000.0))));
    group.bench_function("punnen_big", |b| b.iter(|| punnen(black_box(&graph_big_mut), black_box(1000.0))));
    group.bench_function("berman_big", |b| b.iter(|| berman(black_box(&graph_big_mut), black_box(1000.0))));
    group.bench_function("edge_elm_big", |b| b.iter(|| edge_elimination(black_box(&graph_big_mut), black_box(1000.0))));
    group.bench_function("punnen_mid", |b| b.iter(|| punnen(black_box(&graph_mid_mut), black_box(200.0))));
    group.bench_function("berman_mid", |b| b.iter(|| berman(black_box(&graph_mid_mut), black_box(200.0))));
    group.bench_function("edge_elm_mid", |b| b.iter(|| edge_elimination(black_box(&graph_mid_mut), black_box(200.0))));
    group.bench_function("punnen_small", |b| b.iter(|| punnen(black_box(&graph_small_mut), black_box(100.0))));
    group.bench_function("berman_small", |b| b.iter(|| berman(black_box(&graph_small_mut), black_box(100.0))));
    group.bench_function("edge_elm_small", |b| b.iter(|| edge_elimination(black_box(&graph_small_mut), black_box(100.0))));
    group.bench_function("punnen_very_small", |b| b.iter(|| punnen(black_box(&graph_tiny_mut), black_box(50.0))));
    group.bench_function("berman_very_small", |b| b.iter(|| berman(black_box(&graph_tiny_mut), black_box(50.0))));
    group.bench_function("edge_elm_very_small", |b| b.iter(|| edge_elimination(black_box(&graph_tiny_mut), black_box(50.0))));
    group.finish();
}

fn criterion_benchmark_mbst(c: &mut Criterion) {
    let mut graph_big = InputHandler::read_mut("data/dfn-gwin--D-B-E-N-C-A-N-N_network11_47.json");
    graph_big.inverse_weights();
    let mut group = c.benchmark_group("MBST");
    group.bench_function("MBST", |b| b.iter(|| mbst(black_box(&mut graph_big))));
    group.bench_function("MST", |b| b.iter(|| mst(black_box(&mut graph_big))));
    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    criterion_benchmark_algorithms(c);
    //criterion_benchmark_mbst(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);