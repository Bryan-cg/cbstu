use std::env;
use log::info;
use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use crate::algorithms::constrained_bottleneck_spanning_tree::edge_elimination::EdgeElimination;
use crate::algorithms::constrained_bottleneck_spanning_tree::punnen::Punnen;
use crate::io::input_handler::InputHandler;

mod datastructures;
mod algorithms;
mod tests_functions;
mod io;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    info!("Starting program");
    let graph = InputHandler::read("data/wrp4-11_network123_233.json");
    let neg_graph = graph.negative_weights();
    let (_, _, bottleneck_small_budget_punnen) = EdgeElimination::run(&neg_graph, 100.0);
    info!("Bottleneck small budget: {}", bottleneck_small_budget_punnen);
}
