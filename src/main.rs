use std::env;
use log::info;
use crate::algorithms::constrained_bottleneck_spanning_tree::berman::Berman;
use crate::io::input_handler::InputHandler;

mod datastructures;
mod algorithms;
mod tests_functions;
mod io;

fn main() {
    env::set_var("RUST_LOG", "trace");
    env_logger::init();
    info!("Starting program");
    let graph = InputHandler::read("data/abilene--D-B-M-N-C-A-N-N_network12_15.json");
    let neg_graph = graph.negative_weights();
    //let (_, _, bottleneck_big_budget) = Berman::run(&neg_graph, 10000.0);
    let (_, _, bottleneck_small_budget) = Berman::run(&neg_graph, 100.0);
    //info!("Bottleneck big budget: {}", bottleneck_big_budget);
    info!("Bottleneck small budget: {}", bottleneck_small_budget);
}
